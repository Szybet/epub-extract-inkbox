/*
author ---
pub date ---
cover page location ---
book name ---
*/

use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;

use std::env;
use std::process;

fn main() {
    // Set RUST_LOG=debug to get debug output
    //env_logger::init();
    //debug!("Starting");

    let args: Vec<String> = env::args().collect();
    let args_len = env::args().len();
    //debug!("CLI arguments are: {:?}", args);
    if &args[1] == "--help" || &args[1] == "help" || args_len < 1 {
        println!("First argument: Path to book | Second: Path to extracted ePUB cover (optional). Set RUST_LOG=debug to get debug output.");
        process::exit(1);
    }

    let epub_file = &args[1];

    let mut doc = EpubDoc::new(epub_file).unwrap();

    // Whole hashmap
    //let metadata_all = doc.metadata.clone();
    //debug!("Whole metadata of ePUB: {:?}", metadata_all);

    // Title:
    let title = doc.mdata("title").unwrap();
    //debug!("Title is: {}", title);

    // Cover:
    if args_len > 2 {
        let cover_path = &args[2];
        let cover_data = doc.get_cover().unwrap();
        let f = fs::File::create(cover_path);
        let mut f = f.unwrap();
        let resp = f.write_all(&cover_data).unwrap();
        //debug!("Cover extraction done");
    } else {
        //debug!("Not extracting book cover since no path was provided");
    }

    // Publishing date
    let publish_date = doc.mdata("date").unwrap();
    //debug!("Publishing date: {:?}", publish_date);

    // Author
    let author = doc.mdata("creator").unwrap();
    //debug!("Author: {:?}", author);

    let json = r#"{
    "author": "author_replace",
    "title": "title_replace",
    "date": "date_replace",
}"#;
    let new_json: String = json
        .replace("author_replace", &author)
        .replace("title_replace", &title)
        .replace("date_replace", &publish_date);

    println!("{}", new_json);
}
