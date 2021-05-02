#![feature(associated_type_bounds)]
#![feature(option_result_unwrap_unchecked)]
use futures::{
    stream::{self, BufferUnordered, Iter, Map},
    StreamExt,
}; // 0.3.8

use reqwest::{Client, Error}; // 0.10.9
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use serde::{self, de::IntoDeserializer, Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    vec::IntoIter,
};
use std::{convert::TryInto, str::from_utf8};

#[path = "./types/olx-data.rs"]
pub mod olx_posts;
use olx_posts::OlxPost;

#[macro_use]
extern crate pipeline;
use tokio::{self, task::JoinHandle};
extern crate futures;
use serde_json::{from_str, Value};

#[path = "./types/olx-results.rs"]
pub mod olx_results;
use olx_results::OlxResults;
extern crate quick_xml;

use quick_xml::{
    events::{attributes::Attribute, BytesStart, Event},
    Reader,
};

const CONCURRENT_REQUESTS: usize = 4;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let olx_results = get_olx_results("ps4").await;

    return Ok(());
}
async fn get_olx_results(term: &str) -> Vec<String> {
    let olx_results = search_olx(term).await.expect("Failed to retrieve results");
    let data_json_value = get_data_json(&olx_results);
    let raw_json = serde_json::from_str(&data_json_value);
    let results = filter_search_results(&raw_json.unwrap());
    return results;
}



fn get_data_json(xml: &str) -> String {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref element)) => match element.name() {
                b"script" => {
                    let element_attr = element.attributes().last();
                    println!(
                        "Does data-json exists in attributes? : {:?}",
                        element
                            .attributes()
                            .filter(|attr| attr.as_ref().unwrap().key == b"data-json")
                            .next()
                    );
                    match element_attr {
                        Some(attr) => match attr {
                            Ok(attr) => {
                                if from_utf8(attr.key).unwrap() == "data-json" {
                                    return attr
                                        .unescape_and_decode_value(&reader)
                                        .unwrap();
                                }
                            }
                            Err(_) => {}
                        },
                        None => {}
                    }
                }
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap_or_default()),

            Ok(Event::Eof) => break "".to_string(),
            Err(element) => panic!(
                "Error at position {}: {:?}",
                reader.buffer_position(),
                element
            ),
            _ => (),
        }
        buf.clear();
    }
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
    println!("Filtered results: {:#?}", filtered_results);
    return filtered_results;
}
