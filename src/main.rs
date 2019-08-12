extern crate tendril;
extern crate html5ever;
extern crate curl;

use std::io::{Write};
use curl::easy::Easy;
use std::fs::File;
use std::fs;
use std::env;

fn main() {
    use std::{thread, time};
    use std::path::Path;
    let ten_millis = time::Duration::from_millis(10);
    let url = env::args().nth(1).unwrap();
    loop{
        thread::sleep(ten_millis);
        get_html(&url, "watch_new.html");
        if Path::new("watch.html").is_file() {
            if diff("watch.html", "watch_new.html") {
                println!("changed!");
                fs::remove_file("watch.html").unwrap();
                fs::copy("watch_new.html", "watch.html").unwrap();
            }
            fs::remove_file("watch_new.html").unwrap();
        }

    }
}

fn diff(a_path: &str, b_path: &str) -> bool {
    let a_content = std::fs::read_to_string(a_path).unwrap();
    let b_content = std::fs::read_to_string(b_path).unwrap();
    a_content.eq(&b_content)
}

fn get_html(url: &str, path: &'static str){
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.write_function(move |data| {
        let mut file = File::create(path).unwrap();
        file.write_all(data).unwrap();
        file.flush().unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}
