use indicatif::{ProgressBar, ProgressStyle};
use ansi_term::Color::Blue;
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::env::set_current_dir;

use crate::Cli;

pub fn progress_bar(args: &Cli){
    
    thread::sleep(Duration::from_millis(3000));
    let pg = ProgressBar::new(100);
    pg.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    for _ in 0..100{
      pg.inc(1);
      thread::sleep_ms(28)
    }

    pg.finish_and_clear();

    let path = Path::new(&args.app_name);
    
    let app = set_current_dir(path);


    println!("{}", Blue.bold().paint("Created Flask App 🏁").to_string())
}
