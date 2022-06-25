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
use std::mem::replace;
use std::process;

use sha256::digest_file;
use std::path::Path;

fn main() {
    // Set RUST_LOG=debug to get debug output
    //env_logger::init();
    //debug!("Starting");

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let arg_lenght = args.len();
    //debug!("CLI arguments are: {:?}", args);

    let mut mainString = String::from("[");
    let mainPath = String::from("/data/onboard/.thumbnails/");

    let mut count: usize = 0;
    for epub_file in args {
        count = count + 1;
        let mut doc = EpubDoc::new(&epub_file).unwrap();
        // Whole hashmap
        //let metadata_all = doc.metadata.clone();
        //debug!("Whole metadata of ePUB: {:?}", metadata_all);
        // Title:
        let title = doc.mdata("title").unwrap();
        //debug!("Title is: {}", title);

        // Cover:

        let cover_path = mainPath.clone() + &digest_file(&epub_file).unwrap().to_string();
        let cover_data = doc.get_cover().unwrap();
        let f = fs::File::create(cover_path.clone());
        let mut f = f.unwrap();
        f.write_all(&cover_data).unwrap();
        //debug!("Cover extraction done");

        // Publishing date
        let publish_date = doc.mdata("date").unwrap();
        //debug!("Publishing date: {:?}", publish_date);

        // Author
        let author = doc.mdata("creator").unwrap();
        //debug!("Author: {:?}", author);

        let json = r#"{
            "path": "path_replace",
            "author": "author_replace",
            "title": "title_replace",
            "date": "date_replace"
        }"#;

        let mut new_json: String = json
            .replace("path_replace", &cover_path)
            .replace("author_replace", &author)
            .replace("title_replace", &title)
            .replace("date_replace", &publish_date)
            .replace(" ", "");

        if arg_lenght != count {
            new_json.push_str(",");
        }

        mainString.push_str(&new_json);
    }


    mainString.push_str("]");

    print!("{}", mainString);
}
