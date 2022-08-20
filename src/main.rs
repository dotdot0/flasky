#![allow(unused)]

use clap::{Parser, SubCommand, Subcommand, Args,clap_derive, clap_app};
use std::{fs, io::Write, any::Any};
use std::path::Path;
use std::env::set_current_dir;
use std::{thread, time::Duration};
use std::process::Command;
use indicatif::ProgressBar;

#[derive(Parser,Default)]
struct Cli{
  ///Name of the app
  app_name: String,
  #[clap(short='g', long)]
  ///Initialize Git Repo
  git: bool
}

#[derive(Debug,Subcommand)]
enum Git{
  GitInitialize(GitInitialize)
}

#[derive(Debug,Args)]
struct GitInitialize{
  option: bool
}

fn main(){
  let args = Cli::parse();

  let f = fs::create_dir(&args.app_name).unwrap();

  //let path = Path::new(&args.app_name.trim()).join("/app.py").to_str().unwrap().to_owned();

  let path = format!("{}/app.py", &args.app_name);

  let mut file = fs::File::create(path).expect("ERr");

  let buf = r#"
from flask import Flask

app = Flask(__name__)
@app.route('/')
def index():
  return "Hello World"
  "#.as_bytes();

  file.write(buf);

  if args.git{
    thread::sleep(Duration::from_millis(3000));
    let pg = ProgressBar::new(100);
    for _ in 0..100{
      pg.inc(1);
      thread::sleep_ms(30)
    }
    println!("Creating Flask App");
    let path = Path::new(&args.app_name);
    let app = set_current_dir(path);
    // println!("{:?}", result);
    let result = Command::new("git").args(["init"]).output();
    println!("{:?}", result);
    let result = Command::new("code").args(["."]).output();
  }
}
