/*
author
pub date
cover page location ---
book name ---
*/

use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;

use log::{debug, error, info, log_enabled, Level};
use std::env;
use std::process;
use tinyjson::JsonValue;

fn main() {
    // Set RUST_LOG=debug to get debug output
    env_logger::init();
    debug!("Starting");

    let args: Vec<String> = env::args().collect();
    debug!("CLI arguments are: {:?}", args);
    if &args[1] == "--help" || &args[1] == "help" {
        println!("First argument path to the epub, second is path and name to json file, and the second for the epub cover. Set RUST_LOG=debug to get debug output");
        process::exit(1);
    }
    debug!(
        "arguments are: 1: {}, 2: {}, 3: {}",
        &args[1], &args[2], &args[3]
    );
    let epub_file = &args[1];
    let json_path = &args[2];
    let cover_path = &args[3];

    let mut doc = EpubDoc::new(epub_file).unwrap();

    // The whole hashmap
    let metadata_all = doc.metadata.clone();
    debug!("Whole metadata of this epub: {:?}", metadata_all);

    // title:
    let title = doc.mdata("title").unwrap();
    debug!("tittle is: {}", title);

    // cover:
    let cover_data = doc.get_cover().unwrap();
    let f = fs::File::create(cover_path);
    let mut f = f.unwrap();
    let resp = f.write_all(&cover_data).unwrap();
    debug!("cover done");

    // Publish date
    let publish_date = doc.mdata("date").unwrap();
    debug!("publish date: {:?}", publish_date);

    // Author
    let author = doc.mdata("creator").unwrap();
    debug!("author: {:?}", author);

    let mut json = r#"{
    "author": "author_replace",
    "title": "title_replace",
    "date": "date_replace",
    "cover_path": "cover_replace"
}"#;
    let new_json: String = json
        .replace("author_replace", &author)
        .replace("title_replace", &title)
        .replace("date_replace", &publish_date)
        .replace("cover_replace", &cover_path);

    let f = fs::File::create(json_path);
    let mut f = f.unwrap();
    writeln!(&mut f, "{}", new_json);
}
