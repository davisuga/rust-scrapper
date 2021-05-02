#![feature(associated_type_bounds)]
#![feature(option_result_unwrap_unchecked)]
extern crate futures;
use futures::future::try_join_all; // 0.3.8
use futures::prelude::*;

extern crate regex;
use regex::Regex;

use reqwest::{Client, Error};
use serde_json::Value; // 0.10.9

#[path = "./types/olx-results.rs"]
pub mod olx_results;
use olx_results::OlxResults;

async fn get_results_async(term: &str) -> std::vec::Vec<serde_json::Value> {
    let olx_results = get_olx_urls(term).await;
    let all_results_data = olx_results
        .iter()
        .map(|ad_url| get_ad_data(&ad_url[..]))
        .collect::<Vec<_>>();
    let joined_data = try_join_all(all_results_data).await.unwrap();
    return joined_data;
}

pub fn get_results(term: &str) -> Box<Future<Item = std::vec::Vec<serde_json::Value>>> {
    return get_results_async(term);
}

async fn get_olx_urls(term: &str) -> Vec<String> {
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
