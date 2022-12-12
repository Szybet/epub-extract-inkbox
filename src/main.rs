use epub::doc::EpubDoc;
use std::fs::{self};
use std::io::Write;

use sha256::digest_file;
use std::env;
use std::path::Path;

fn main() {
    let mut extract_cover = true;
    let mut args: Vec<String> = env::args().collect();

    match env::var("EXTRACT_COVER") {
        Ok(v) => {
            if v == "1" || v == "true" {
                extract_cover = true
            }
        }
        Err(_) => extract_cover = false,
    }

    args.remove(0);

    let arg_length = args.len();

    let mut main_string = String::from(
        r#"{
            "database":[
            "#,
    );

    let thumbnails_path = "/mnt/onboard/onboard/.thumbnails/";
    std::fs::create_dir_all(thumbnails_path).unwrap();
    let main_path = String::from(thumbnails_path);

    let mut count: usize = 0;
    for epub_file in args {
        count += 1;
        let mut doc_res = EpubDoc::new(&epub_file);

        if doc_res.is_ok() {
            // Title
            let mut doc = doc_res.unwrap();
            let title = doc.mdata("title").unwrap_or("No title found".to_string());

            // Cover
            let file_digest = digest_file(&epub_file).unwrap().to_string();
            let mut cover_path_converted = main_path.clone() + &file_digest;
            let cover_path = cover_path_converted.clone() + ".t";
            if !Path::new(&cover_path_converted).exists() && extract_cover {
                let cover_data = doc.get_cover();
                if cover_data.is_ok() {
                    let f = fs::File::create(cover_path.clone());
                    let mut f = f.unwrap();
                    f.write_all(&cover_data.unwrap()).unwrap();
                    // No explanation to this
                    if !Path::new(&cover_path).exists() {
                        cover_path_converted = String::from("");
                    }
                } else {
                    cover_path_converted = String::from("");
                }
            }

            // Publication date
            let publication_date = doc.mdata("date").unwrap();

            // Author
            let author = doc.mdata("creator").unwrap();

            let json = r#"{
            "BookID": "book_id_replace",
            "BookPath": "book_path_replace",
            "CoverPath": "cover_path_replace",
            "Author": "author_replace",
            "Title": "title_replace",
            "PublicationDate": "publication_date_replace"
        }"#;

            let mut new_json: String = json
                .replace("book_id_replace", &count.to_string())
                .replace("book_path_replace", &epub_file)
                .replace("cover_path_replace", &cover_path_converted)
                .replace("author_replace", &author)
                .replace("title_replace", &title)
                .replace("publication_date_replace", &publication_date);

            if arg_length != count {
                new_json.push(',');
            }
            main_string.push_str(&new_json);
        } else {
            // Leave this error, its important
            eprintln!("Failed to init epub. Its propably broken: {}", epub_file);
        }
    }
    print!("{}", main_string);
}
