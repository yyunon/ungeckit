use std::{thread, time, fs};
use serde_json;
use ungeckit::driver::*;
use ungeckit::*;
use ungeckit::utils::net::ws;
use simplelog::*;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
    if(fs::exists("intoli.png").unwrap()) {
        fs::remove_file("intoli.png").unwrap();
        save_screenshot!("https://intoli.com/blog/not-possible-to-block-chrome-headless/chrome-headless-test.html", "intoli.png");
    }
    if(fs::exists("sannysoft.png").unwrap()) {
        fs::remove_file("sannysoft.png").unwrap();
        save_screenshot!("https://bot.sannysoft.com/", "sannysoft.png");
    }

    
    // In case timing is needed!
    //let ten_millis = time::Duration::from_millis(5000);
    //let now = time::Instant::now();
    //thread::sleep(ten_millis);

}
