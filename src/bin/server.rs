#[macro_use]
extern crate json;

use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use bytes::BytesMut;
// use futures::{Future, Stream};
// use json::JsonValue;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Single {
    class: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Mentioned {
    class: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CanTake {
    class: String,
    history: Vec<String>,
}

/// This handler uses json extractor
fn metioned_in(item: web::Json<Mentioned>) -> HttpResponse {
    println!("model: {:?}", &item);

    let filename = "./results.json";
    let mut content = String::new();
    let mut f = File::open(filename.clone()).unwrap();
    f.read_to_string(&mut content).unwrap();
    let p: Vec<CrseLevel> = serde_json::from_str(&content).unwrap();

    let res: Vec<CrseLevel> = p
        .iter()
        .filter(|crse| {
            //
            // crse.crse == "MAT 242"
            let mut keep = false;
            for prq in crse.list_pre_reqs.clone() {
                for px in prq.mentioned {
                    keep = px == item.class;
                }
            }
            keep
        })
        .cloned()
        .collect();

    HttpResponse::Ok().json(res) // <- send response
                                 // HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor
fn single_class(item: web::Json<Single>) -> HttpResponse {
    println!("model: {:?}", &item);

    let filename = "./results.json";
    let mut content = String::new();
    let mut f = File::open(filename.clone()).unwrap();
    f.read_to_string(&mut content).unwrap();
    let p: Vec<CrseLevel> = serde_json::from_str(&content).unwrap();

    let res: Vec<CrseLevel> = p
        .iter()
        .filter(|crse| crse.crse == item.class)
        .cloned()
        .collect();

    HttpResponse::Ok().json(res) // <- send response
                                 // HttpResponse::Ok().json(item.0) // <- send response
}

#[derive(Debug, Serialize, Deserialize)]
struct CanYouTakeResponse {
    can_take: bool,
    did_not_complete: Vec<String>,
    did_complete: Vec<String>,
}

/// This handler uses json extractor
fn can_i_take(item: web::Json<CanTake>) -> HttpResponse {
    println!("model: {:?}", &item);

    if item.class == "" || item.class.len() < 7 {
        return HttpResponse::Ok().json({});
    };

    let filename = "./results.json";
    let mut content = String::new();
    let mut f = File::open(filename.clone()).unwrap();
    f.read_to_string(&mut content).unwrap();
    let p: Vec<CrseLevel> = serde_json::from_str(&content).unwrap();

    let class_history: Vec<String> = item.history.clone();

    let wantclasses: Vec<CrseLevel> = p
        .iter()
        .filter(|crse| crse.crse == item.class.to_uppercase())
        .cloned()
        .collect();

    let wantclass: CrseLevel = wantclasses.first().unwrap().clone();

    let mut did_complete_reqs = false;

    let mut current_item = vec![];
    let mut did_not_complete = vec![];
    for prq in wantclass.list_pre_reqs.clone() {
        for px in &prq.mentioned {
            // keep = px == "MAT 210"
            if class_history.contains(&px) {
                did_complete_reqs = true;
                current_item.push(px.clone())
            } else {
                did_not_complete.push(px.clone())
            };
        }
    }

    let response = CanYouTakeResponse {
        can_take: did_complete_reqs,
        did_not_complete: did_not_complete,
        did_complete: current_item,
    };

    HttpResponse::Ok().json(response) // <- send response
                                      // HttpResponse::Ok().json(item.0) // <- send response
}

// const MAX_SIZE: usize = 262_144; // max payload size is 256k

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            // .service(web::resource("/extractor").route(web::post().to(index)))
            .service(web::resource("/").route(web::get().to(homepage)))
            .service(web::resource("/metioned_in").route(web::post().to(metioned_in)))
            .service(web::resource("/single_class").route(web::post().to(single_class)))
            .service(web::resource("/can_i_take").route(web::post().to(can_i_take)))
    })
    .bind("127.0.0.1:9090")?
    .run()
}

use actix_web::http::StatusCode;
use serde_json::json;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PreReqClassified {
    is_min: bool,
    is_max: bool,
    is_grad: bool,
    is_non_degree_seeking: bool,
    is_degree_seeking: bool,
    is_freshy: bool,
    is_about_credit: bool,
    is_barrett_honors: bool,
    is_c_or_better: bool,
    is_additional_hours: bool,
    is_cannot_enroll: bool,
    is_earned_credit_hours: bool,
    is_mary_lou_fulton: bool,
    is_teachers: bool,
    is_soph: bool,
    is_junior: bool,
    is_senior: bool,
    is_mathematics_placement_test: bool,
    is_aleks: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PreReq {
    original: String,
    classified: PreReqClassified,
    mentioned: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CrseLevel {
    index: i32,
    crse: String,
    input: String,
    list_pre_reqs: Vec<PreReq>,
}

fn homepage() -> HttpResponse {
    let filename = "./src/index.html";
    let mut content = String::new();
    let mut f = File::open(filename.clone()).unwrap();
    f.read_to_string(&mut content).unwrap();

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(content)
}

// fn homepage() -> HttpResponse {
//     HttpResponse::build(StatusCode::OK)
//         .content_type("text/html; charset=utf-8")
//         .body(
//             r#"
//     "#,
//         )
// }
