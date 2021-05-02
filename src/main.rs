#![feature(associated_type_bounds)]
#![feature(option_result_unwrap_unchecked)]
use futures::future::try_join_all; // 0.3.8
extern crate regex;
use regex::Regex;

use reqwest::{Client, Error}; // 0.10.9
use tokio;
extern crate futures;
use serde_json::Value;

#[path = "./types/olx-results.rs"]
pub mod olx_results;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use olx_results::OlxResults;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
async fn get_olx_results_data(term: &str) {
    let olx_results = get_olx_results(term).await;
    let all_results_data = olx_results
        .iter()
        .map(|ad_url| get_ad_data(&ad_url[..]))
        .collect::<Vec<_>>();
    let joined_data = try_join_all(all_results_data).await.unwrap();
    println!("{:?}", joined_data);
}

async fn get_olx_results(term: &str) -> Vec<String> {
    let olx_results = search_olx(term).await.expect("Failed to retrieve results");
    let data_json_value = get_data_json(&olx_results);
    println!("trying to parse: {}", &data_json_value);
    let raw_json = serde_json::from_str(&data_json_value);
    match raw_json {
        Ok(valid_json) => filter_search_results(&valid_json),
        Err(_err) => vec!["".to_string()],
    }
}

async fn get_ad_data(link: &str) -> Result<Value, serde_json::Error> {
    let client = Client::new();
    let result = client.get(link).send().await.unwrap();
    let text_result = result.text().await.unwrap();
    let raw_data = get_data_json(&text_result[..]);
    let json_data: Result<Value, serde_json::Error> = serde_json::from_str(&raw_data[..]);
    println!("parsed: {:?}", json_data);
    return json_data;
}

fn get_data_json(xml: &str) -> String {
    let regex = Regex::new(r#"data-json="(.+)">.+/script>"#).unwrap();
    let result = regex.captures(xml);
    let parsed_result = &result.unwrap()[0];
    let begin = 11;
    let end = parsed_result.len() - 11;
    let sliced_result = &parsed_result[begin..end];
    let unescaped_result = str::replace(sliced_result, r"&quot;", r#"""#);
    return String::from(unescaped_result);
}

async fn search_olx(term: &str) -> Result<String, Error> {
    let olx_search_url = format!("https://mg.olx.com.br/belo-horizonte-e-regiao?q={}", term);
    let client = Client::new();
    let result = client.get(&olx_search_url).send().await.unwrap();
    let text_result = result.text().await;
    return text_result;
}

fn filter_search_results(search_results: &OlxResults) -> Vec<String> {
    println!(
        "First url: {:#?}",
        search_results
            .listing_props
            .ad_list
            .iter()
            .nth(0)
            .unwrap()
            .url
    );
    let filtered_results: Vec<String> = search_results
        .listing_props
        .ad_list
        .iter()
        .filter(|ad| ad.url.is_some())
        .map(|safe_ad| return safe_ad.url.clone().unwrap())
        .collect::<Vec<_>>();
    return filtered_results;
}
