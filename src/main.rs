use futures::{
    stream::{self, BufferUnordered, Iter, Map},
    StreamExt,
}; // 0.3.8

use reqwest::{Client, Error}; // 0.10.9
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use serde::{self, Deserialize, Serialize};

use std::{borrow::Borrow, vec::IntoIter};

#[path = "./types/olx-data.rs"]
pub mod olx_posts;
use olx_posts::OlxPost;

#[macro_use]
extern crate pipeline;
use tokio::{self, task::JoinHandle};
extern crate futures;
use serde_json::Value;

#[path = "./types/olx-results.rs"]
pub mod olx_results;
use olx_results::OlxResults;
extern crate quick_xml;

use quick_xml::{
    events::{attributes::Attribute, Event},
    Reader,
};

const CONCURRENT_REQUESTS: usize = 4;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let olx_results = search_olx("escorredor")
        .await
        .expect("Failed to retrieve results");
    let olx_data = parse_olx_page::<OlxResults>(&olx_results);
    println!("OLX DATA: {:#?}", olx_data);

    return Ok(());
}

fn parse_script_tag<T: Deserialize<'static>>(
    tag: select::document::Select<'static, Attr<&str, &str>>,
) -> Option<T> {
    let mut nodes = tag.map(|tag| {
        let json = tag.attr("data-json").unwrap();
        let parsed: T = serde_json::from_str(json).expect("Failed to parse json");
        return parsed;
    });
    // .collect::<Vec<_>>();

    let possible_script_tag = nodes.next();
    return possible_script_tag;
}
fn get_safe_data_json_value(
    possible_attr: Option<Result<Attribute, quick_xml::Error>>,
) -> Option<Attribute> {
    match possible_attr {
        Some(Ok(safe)) => Some(safe),
        Some(Err(err)) => {
            println!("Failed to parse json: {:?}", err);
            None
        }
        None => None,
    }
}
fn parse_olx_page<T: Deserialize<'static>>(xml: &str) -> Option<T> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut count = 0;
    let mut txt: Vec<T> = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref element)) => match element.name() {
                b"script" => {
                    let element_attr = element.attributes().last();
                    println!("Element attr: {:#?}", element_attr);
                    get_safe_data_json_value(element_attr);
                }
                _ => (),
            },
            Ok(Event::Eof) => break None,
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

fn output_search(search_results: Option<Value>) {
    match search_results {
        Some(results) => println!("Got {}", results),
        None => println!("No results"),
    }
}

fn filter_search_results(search_results: Option<&OlxResults>) {
    match search_results {
        Some(results) => {
            println!("{:#?}", results);
            let filtered_results = results
                .listing_props
                .ad_list
                .iter()
                //     .filter(|result| result["price"].is_string());
                // let f = filtered_results
                .map(|ad| {
                    println!("GOT {:?}", ad.url)
                    // println!(
                    //     "Title: {:?} \n Price: {:?} \n   Link: {:?}",
                    //     ad["subject"], ad["price"], ad["url"]
                    // )
                });
        }
        None => println!("No results"),
    }
}
