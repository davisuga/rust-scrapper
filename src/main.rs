use futures::{stream, StreamExt}; // 0.3.8
use reqwest::Client; // 0.10.9
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use tokio; // 0.2.24, features = ["macros"]
extern crate futures;
const CONCURRENT_REQUESTS: usize = 4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let links:Vec<&str> = vec!["https://mg.olx.com.br/belo-horizonte-e-regiao/computadores-e-acessorios/monitor-curvo-samsung-24-fhd-hdmi-vga-freesync-preto-serie-cf390-853157530", "https://mg.olx.com.br/belo-horizonte-e-regiao/computadores-e-acessorios/monitor-27-philips-widescreen-4k-uhd-ips-mostruario-nota-garantia-838877513"];
    let client = Client::new();
    let raw_buffer = stream::iter(links)
        .map(|link| {
            let client = client.clone();
            tokio::spawn(async move {
                println!("spawning task...");
                let resp = client.get(link).send().await?;
                resp.text().await
            })
        })
        .buffer_unordered(CONCURRENT_REQUESTS);
    raw_buffer
        .for_each(|result| async {
            match result {
                Ok(Ok(result)) => {
                    println!("Got {} bytes", result.len());
                    parse_html(result)
                }
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
    Ok(())
}
// fn handle_response(response: Response) {}
fn parse_html(document_string: String) {
    let document_slice_string = &document_string[..];
    let document = Document::from(document_slice_string);
    let script_tags = document.select(Attr("id", "initial-data"));
    let nodes = script_tags.map(|tag| tag.as_text());
    println!("{:#?}", nodes.count());
}
