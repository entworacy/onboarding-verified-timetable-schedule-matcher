
pub mod schedule {
    use std::error::Error;
    use lingua::Language::{English, Japanese, Korean};
    use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
    pub use crate::model::DateGroupList;
    use crate::model::{CommonCollaborator, CommonSchedule, ExpectedLanguageInfo, HttpError, NijisanjiResponse, PlatformType};
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

        pub async fn get_timetable_hololive(&self) -> Result<DateGroupList, HttpError> {
            let mut resp = reqwest::get("https://schedule.hololive.tv/api/list/7")
                .await;

            if resp.is_err() {
                return Err(HttpError::new("요청처리에 문제가 발생했습니다. 잠시 후 다시 시도해주세요.", 0));
            }

            let resp = resp.unwrap().json::<DateGroupList>().await;

            if resp.is_err() {
                return Err(HttpError::new("요청처리에 문제가 발생했습니다. 잠시 후 다시 시도해주세요.", resp.err().unwrap().status().unwrap().as_u16()));
            }
            Ok(resp.unwrap())
        }

        pub async fn get_timetable_nijisanji(&self) -> NijisanjiResponse {
            let resp = reqwest::get("https://api.itsukaralink.jp/v1.2/events.json")
                .await.unwrap()
                .json::<NijisanjiResponse>().await.unwrap();
            resp
        }

        pub async fn get_timetable(&self, entertainment_type: EntertainmentType) -> Result<Vec<CommonSchedule>, HttpError> {
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
                    let _date_groups = self.get_timetable_hololive().await;
                    match _date_groups {
                        Ok(date_groups) => {

                            for date_group in date_groups.dateGroupList {
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

                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                EntertainmentType::NijiSanji => {
                    return Err(HttpError::new("현재 지원하지 않는 MCN입니다.2022년 하반기중 지원예정이오니 조금만 기다려주세요.자세한 사항은 고객센터로 문의바랍니다.",0));
                }
                EntertainmentType::Other => {}
            }

            Ok(schedule_info_list)
        }
    }
}