use std::borrow::Borrow;
use tokio::net::TcpListener;
mod model;
mod schedule;

//use onboarding_verified_timetable_schedule_matcher::schedule::schedule::EntertainmentType::{HoloLive, NijiSanji};
//use onboarding_verified_timetable_schedule_matcher::schedule::schedule::TimetableParser;
extern crate redis;
use redis::Commands;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use crate::schedule::schedule::EntertainmentType::{HoloLive, NijiSanji};
use crate::schedule::schedule::TimetableParser;

#[get("/api/mcn-timetable/default-json/hololive.json")]
async fn get_hololive_default_api_response() -> HttpResponse {

    let ti = TimetableParser::new();
    let mcn_response = ti.get_timetable_hololive().await;
    return match mcn_response {
        Ok(response) => {
            HttpResponse::Ok()
                .json(json!(
                    {
                        "success": true,
                        "message": "요청사항을 성공적으로 처리하였습니다.",
                        "data": response
                    }
                ))
        }

        Err(error) => {
            HttpResponse::InternalServerError()
                .json(json!(
                    {
                        "success": false,
                        "message": "처리되지 못했습니다.",
                        "errorCode": 304,
                        "errorMessage": error.message,
                        "detailError": "일시적인 문제일 수 있으니 잠시 후에 재시도해주시기 바랍니다."
                    }
                ))
        }
    }
}

#[get("/api/mcn-timetable/{mcn}/{mcn_code}/get_default_timetable.json")]
async fn get_mcn_timetable(mcn: web::Path<(String, String)>) -> HttpResponse {
    let ti = TimetableParser::new();
    let mut mcn_type = HoloLive;
    let mcn_path = mcn.into_inner();
    if !["HoloLive Production", "Nijisanji Network"].contains(&&**&mcn_path.0) {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&json!(
            {
                "success": false,
                "message": "올바르지 않은 요청입니다.(MCN-filter)",
                "errorMessage": "MCN 정보가 정상적이지 않아 일시적으로 이 메뉴를 이용할 수 없습니다. 일시적인 장애일 수 있으니 잠시 후 다시 시도해주세요.",
                "errorCode": 82,
                "detailError": "해당 MCN이 존재하는지 다시 한번 확인해주시기 바랍니다."
            })).unwrap());
    } else {
        match mcn_path.0.as_str() {
            "Nijisanji Network" => mcn_type = NijiSanji,
            _ => {}
        }
    }

    let res = ti.get_timetable(
        mcn_type, mcn_path.1.as_str()
    ).await;

    return match res {
        Ok(response) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&json!(
                    {
                        "success": true,
                        "message": "요청사항을 성공적으로 처리하였습니다.",
                        "scheduleList": response
                    }
                )).unwrap())
        }

        Err(error) => {
            HttpResponse::InternalServerError()
                .json(json!(
                    {
                        "success": false,
                        "message": "처리되지 못했습니다.",
                        "errorCode": 304,
                        "errorMessage": error.message,
                        "detailError": "일시적인 문제일 수 있으니 잠시 후에 재시도해주시기 바랍니다."
                    }
                ))
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_mcn_timetable).service(get_hololive_default_api_response)
            .wrap(actix_web::middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*")))
    })
        .bind(("127.0.0.1", 4353))?
        .run()
        .await
}