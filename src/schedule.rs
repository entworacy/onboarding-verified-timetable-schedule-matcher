
pub mod schedule {
    use lingua::Language::{English, Japanese, Korean};
    use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
    pub use crate::model::DateGroupList;
    use crate::model::{CommonCollaborator, CommonSchedule, ExpectedLanguageInfo, NijisanjiResponse, PlatformType};
    use crate::schedule::schedule::EntertainmentType::Other;

    pub enum EntertainmentType {
        HoloLive, NijiSanji, Other
    }

    pub struct TimetableParser {
        pub group: DateGroupList,
        detector: LanguageDetector
    }

    impl TimetableParser {
        pub fn new() -> Self {
            Self {
                group: DateGroupList {
                    dateGroupList: vec![],
                },
                detector: LanguageDetectorBuilder::from_languages(&[English, Korean, Japanese])
                    .with_minimum_relative_distance(0.5)
                    .build()
            }
        }

        pub async fn get_timetable_hololive(&self) -> DateGroupList {
            let resp = reqwest::get("https://schedule.hololive.tv/api/list/7")
                .await.unwrap()
                .json::<DateGroupList>().await
                .unwrap();
            resp
        }

        pub async fn get_timetable_nijisanji(&self) -> NijisanjiResponse {
            let resp = reqwest::get("https://api.itsukaralink.jp/v1.2/events.json")
                .await.unwrap()
                .json::<NijisanjiResponse>().await.unwrap();
            resp
        }

        pub async fn get_timetable(&self, entertainment_type: EntertainmentType) -> Vec<CommonSchedule> {
            let mut schedule_info_list: Vec<CommonSchedule> = vec![];
            let mut schedule_info: CommonSchedule = CommonSchedule {
                platform_type: 2,
                contents_title: "".to_string(),
                contents_author_name: "".to_string(),
                contents_author_profile_url: "".to_string(),
                start_date: "".to_string(),
                url: "".to_string(),
                contents_thumbnail: None,
                has_collaborator: false,
                collaborators: vec![],
                expected_streamer_language: vec![]
            };


            match entertainment_type {
                EntertainmentType::HoloLive => {
                    let date_groups = self.get_timetable_hololive().await.dateGroupList;
                    for date_group in date_groups {
                        for video in date_group.videoList {
                            schedule_info.platform_type = 0;
                            schedule_info.has_collaborator = !video.collaboTalents.is_empty();
                            schedule_info.collaborators = vec![];
                            for collaborator_talent in video.collaboTalents {
                                schedule_info.collaborators.push(
                                    CommonCollaborator {
                                        collaborator_name: None,
                                        collaborator_profile_url: collaborator_talent.iconImageUrl
                                    }
                                );
                            }
                            schedule_info.start_date = video.datetime;
                            schedule_info.contents_author_name = video.name;
                            schedule_info.contents_thumbnail = Option::from(video.thumbnail);
                            schedule_info.contents_title = video.title;
                            schedule_info.contents_author_profile_url = video.talent.iconImageUrl;
                            schedule_info.url = video.url;

                            schedule_info.expected_streamer_language = vec![];

                            let detected_language_for_title = self.detector.compute_language_confidence_values(schedule_info.contents_title.clone());
                            if !detected_language_for_title.is_empty() && detected_language_for_title.first().is_some() {
                                let (lang, accuracy) = detected_language_for_title.first().unwrap();
                                if accuracy < &0.5 {
                                    schedule_info.expected_streamer_language.push(
                                        ExpectedLanguageInfo {
                                            expected_target: "제목".to_string(),
                                            expected_language: None,
                                            accuracy: *accuracy
                                        }
                                    )
                                } else {
                                    schedule_info.expected_streamer_language.push(
                                        ExpectedLanguageInfo {
                                            expected_target: "제목".to_string(),
                                            expected_language: Option::from(match lang {
                                                Korean => "한국어",
                                                English => "영어",
                                                Japanese => "일본어",
                                                _ => "알 수 없음"
                                            }.to_string()),
                                            accuracy: *accuracy
                                        }
                                    )
                                }
                            }
                            schedule_info_list.push(schedule_info.clone());
                        }
                    }
                }
                EntertainmentType::NijiSanji => {

                }
                EntertainmentType::Other => {}
            }

            schedule_info_list
        }
    }
}