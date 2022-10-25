use std::borrow::Borrow;
use tokio::net::TcpListener;
use onboarding_verified_timetable_schedule_matcher::schedule::schedule::EntertainmentType::{HoloLive, NijiSanji};
use onboarding_verified_timetable_schedule_matcher::schedule::schedule::TimetableParser;
extern crate redis;
use redis::Commands;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/api/mcn-timetable/{mcn}/get_default_timetable.json")]
async fn get_mcn_timetable(mcn: web::Path<String>) -> HttpResponse {
    let ti = TimetableParser::new();
    let mut mcn_type = HoloLive;
    let mcn_path = mcn.into_inner();
    if !["hololive", "niji"].contains(&&**&mcn_path) {
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(serde_json::to_string(&json!(
            {
                "success": false,
                "message": "MCN 타입이 올바르지 않습니다. 일시적인 장애일 수 있으니 잠시 후 다시 시도해주세요.",
                "errorCode": 82
            })).unwrap());
    } else {
        match mcn_path.as_str() {
            "niji" => mcn_type = NijiSanji,
            _ => {}
        }
    }

    let res = ti.get_timetable(
        mcn_type
    ).await;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&json!(
            {
                "success": true,
                "message": "요청사항을 성공적으로 처리하였습니다.",
                "scheduleList": res
            }
        )).unwrap())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_mcn_timetable)
    })
        .bind(("127.0.0.1", 4353))?
        .run()
        .await
}