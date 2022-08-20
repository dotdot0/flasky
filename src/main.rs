#![allow(unused)]

use clap::Parser;
use std::{fs, io::Write, any::Any};
use std::path::Path;


#[derive(Parser)]
struct Cli{
  ///Name of the app
  app_name: String
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
}
