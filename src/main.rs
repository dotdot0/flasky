#![allow(unused, deprecated)]

//mod
mod progress;

//Use
use clap::{Parser, SubCommand, Subcommand, Args,clap_derive, clap_app};
use std::default;
use std::{fs, io::Write, any::Any};
use std::path::Path;
use std::env::{set_current_dir, args};
use std::{thread, time::Duration};
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use ansi_term::Colour::{Blue, Yellow};
use progress::progress_bar;

#[derive(Parser)]
#[clap(version, about="Create boilerplate for new flask app and more..")]
pub struct Cli{

  ///Name of the app
  app_name: String,
  #[clap(short='g', long)]

  ///Initialize Git Repo
  git: bool,

  #[clap(short='c', long)]
  ///Open the project in visual studio code
  code: bool,

  #[clap(short='t', long)]
  ///Create the jinja2 templates directory
  templates: bool,

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


  progress_bar(&args);

  if args.git{
    let result = Command::new("git").args(["init"]).output();
     
  }

  if args.code{

    let result = Command::new("code").args(["."]).output();
  }

  if args.templates{

    let template = fs::create_dir("templates").unwrap();
    thread::sleep(Duration::from_millis(500));
    let mut file = fs::File::create("templates/home.html").expect("Unable To Create File");

    let html: &[u8] = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Flasky</title>
    </head>
    <body>
        <h1>Hello World!</h1>
    </body>
</html>
    "#.as_bytes();

  file.write(html);
  }

  //if args.nvim{
    //let result = Command::new("nvim").args(args).output();
  //}
}



