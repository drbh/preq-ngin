extern crate regex;
use regex::Regex;
extern crate walkdir;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    for entry in WalkDir::new("./pages/")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        // println!("{:?}", entry);
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;
        if f_name.ends_with(".html") && sec.elapsed()?.as_secs() < 86400 {
            let filename = format!("./pages/{fname}", fname = f_name);

            let mut content = String::new();
            let mut f = File::open(filename.clone())?;
            f.read_to_string(&mut content).unwrap();

            let fragment = Html::parse_document(&content);
            let prereq_selector = Selector::parse("div.enrollReq").unwrap();
            let title_selector = Selector::parse("td.subjectNumberColumnValue > span").unwrap();

            let mut elcount = 0;
            let a = fragment.select(&prereq_selector);
            let b = fragment.select(&title_selector);
            for it in a.zip(b) {
                let (ai, bi) = it;
                elcount += 1;
                let class_title = bi.inner_html().trim().to_string();
                let preq_text = ai.inner_html();

                let pattern = Regex::new(r"[^A-Za-z0-9 ;:]").unwrap();
                // let pattern = Regex::new(r"").unwrap();

                let newline: String = pattern.replace_all(&preq_text, "").to_string();

                println!("{}++{}++{}", elcount, class_title, newline);
            }
        }
        count += 1;
        // if count > 5 {
        //     break;
        // }
    }

    Ok(())
}
