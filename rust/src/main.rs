use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Message {
    id: String,
    from_email: String,
    from_name: String,
    html_body: String,
}

fn load_messages<P: AsRef<Path>>(path: P) -> Result<Vec<Message>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let messages: Vec<Message> = serde_json::from_reader(reader)?;

    Ok(messages)
}

fn extract_links(html: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let links = dom
        .query_selector("a[href]")
        .unwrap()
        .map(|e| e.get(parser).and_then(tl::Node::as_tag).unwrap())
        .map(|e| {
            e.attributes()
                .get("href")
                .flatten()
                .and_then(tl::Bytes::try_as_utf8_str)
                .unwrap()
                .to_string()
        })
        .collect();

    Ok(links)
}

fn main() -> std::io::Result<()> {
    let now = Instant::now();

    let messages = load_messages("../messages.json").unwrap();
    let mut all_links: Vec<String> = vec![];

    for message in messages {
        let mut links = extract_links(&message.html_body).unwrap();
        all_links.append(&mut links);
    }

    let mut file = File::create("links.txt")?;
    let contents = all_links.join("\n");
    file.write_all(contents.as_bytes())?;

    let elapsed = now.elapsed();
    println!("time: {:?}ms", elapsed.as_millis());

    Ok(())
}
