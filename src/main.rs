use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;

use std::env;
use sha256::digest_file;
use std::path::Path;

fn main() {
    let mut extract_cover = true;
    let mut args: Vec<String> = env::args().collect();

    match env::var("EXTRACT_COVER") {
        Ok(_v) => extract_cover = true,
        Err(_e) => extract_cover = false
    }

    args.remove(0);

    let arg_length = args.len();

    let mut main_string = String::from(r#"{
            "database":[
            "#);

    let thumbnails_path = "/mnt/onboard/onboard/.thumbnails/";
    std::fs::create_dir_all(thumbnails_path);
    let main_path = String::from(thumbnails_path);

    let mut count: usize = 0;
    for epub_file in args {
        count = count + 1;
        let mut doc = EpubDoc::new(&epub_file).unwrap();

        // Title
        let title = doc.mdata("title").unwrap();

        // Cover
        let file_digest = digest_file(&epub_file).unwrap().to_string();
        let mut cover_path = main_path.clone() + &file_digest + ".t";
        let cover_path_converted = main_path.clone() + &file_digest;
        if !Path::new(&cover_path_converted).exists() {
            if extract_cover == true {
                let cover_data = doc.get_cover();
                if cover_data.is_ok() {
                    let f = fs::File::create(cover_path.clone());
                    let mut f = f.unwrap();
                    f.write_all(&cover_data.unwrap()).unwrap();
                }
                else {
                    cover_path = "".to_string();
                }
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
            .replace("cover_path_replace", &cover_path)
            .replace("author_replace", &author)
            .replace("title_replace", &title)
            .replace("publication_date_replace", &publication_date);

        if arg_length != count {
            new_json.push_str(",");
        }
        main_string.push_str(&new_json);
    }
    print!("{}", main_string);
}
