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
    let olx_results = search_olx("escorredor")
        .await
        .expect("Failed to retrieve results");
    let data_json_value = parse_olx_page(&olx_results);
    let raw_json: OlxResults = serde_json::from_str(&data_json_value).unwrap();
    filter_search_results(Some(&raw_json));
    // println!("OLX DATA: {:#?}", olx_data);

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

// fn get_safe_data_json_value<T: Deserialize<'static>>(
//     possible_attr: Option<Result<Attribute, quick_xml::Error>>,
// ) -> Option<T> {
//     match possible_attr {
//         Some(Ok(safe)) => {
//             println!("True value: {:#?}", safe.value);
//             let cow = safe.unescaped_value().unwrap_or_default();
//             match from_utf8(&cow) {
//                 Ok(safe_string) => {
//                     Some(safe_string.to_string());
//                     let json: T =
//                         serde_json::from_str(safe_string).expect("Error while decoding json");
//                     return Some(json);
//                 }
//                 Utf8Error => None,
//                 Err(_) => None,
//             }
//         }
//         Some(Err(err)) => {
//             println!("Failed to parse json: {:?}", err);
//             None
//         }
//         None => None,
//     }
// }

// fn get_data_json_from_element(element: &BytesStart) -> Option<Cow<'_, [u8]>> {
//     let a = element
//         .attributes()
//         .map(|attr| {
//             if attr.as_ref().unwrap().key == b"data-json" {
//                 let sdfa = attr.as_ref().unwrap().value;
//                 return sdfa;
//             } else {
//                 return None;
//             }
//         })
//         .next();
//     return a.unwrap();
// }
fn parse_olx_page(xml: &str) -> String {
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
                                        .unwrap_or_default();
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
    // let real_value = txt.iter().map(|json_value| {
    //     println!("JSON shit: {:#?}", json_value);
    //     println!("got into the iterator");
    //     return serde_json::from_str::<'static, String>(json_value);
    // });
    // print!("real value{:#?}", real_value);
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

fn filter_search_results(search_results: Option<&OlxResults>) -> Option<Vec<&String>> {
    match search_results {
        Some(results) => {
            println!(
                "First url: {:#?}",
                results.listing_props.ad_list.iter().nth(0).unwrap().url
            );
            let filtered_results: Vec<&String> = results
                .listing_props
                .ad_list
                .iter()
                //     .filter(|result| result["price"].is_string());
                // let f = filtered_results
                .filter(|ad| ad.url.as_ref().is_some())
                .map(|safe_ad| return safe_ad.url.as_ref().unwrap())
                .collect::<Vec<_>>();
            println!("Filtered results: {:#?}", filtered_results);
            return Some(filtered_results);
        }
        None => {
            println!("No results");
            return None;
        }
    }
}
