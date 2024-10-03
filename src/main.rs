use std::fs;
use std::io::{Read, Write};

use clap::{Arg, ArgAction, Command};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Url;

fn main() {
    let matches = Command::new("rs-get")
        .version("0.1.0")
        .author("redacted")
        .about("wget clone w rusty")
        .args([
            Arg::new("url")
                .short('u')
                .help("URL to download")
                .required(true),
            Arg::new("quiet")
                .short('q')
                .help("screamn't when getting stuff")
                .required(false)
                .action(ArgAction::SetFalse)
        ]).get_matches();

    if let Some(url) = matches.get_one::<String>("url") {
        match matches.get_flag("quiet") {
            true => {
                let _ = download(url, true);
            },
            false => {
                let _ = download(url, false);
            },
        };
    } else {
        eprintln!("invalid url or parsing error");
    }
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        },
    };

    bar.set_message(msg.to_string());

    match length.is_some() {
        true => bar.set_style(ProgressStyle::default_bar()),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}

fn download(target: &str, quiet_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(target)?;
    let client = Client::new();
    let mut resp = client.get(url).send()?;

    if !quiet_mode {
        println!("HTTP request sent... {}", resp.status());
    }

    if resp.status().is_success() {
        let headers = resp.headers().clone();

        let content_len: Option<u64> = headers
            .get(CONTENT_LENGTH)
            .and_then(|val| val.to_str().ok())
            .and_then(|str| str.parse().ok());

        let content_type: Option<String> = headers
            .get(CONTENT_TYPE)
            .and_then(|val| val.to_str().ok())
            .and_then(|str| str.parse().ok());

        let mut filename = target.split("/").last().unwrap();

        if filename.is_empty() {
            filename = "index.html";
        }

        let chunk_size = match content_len {
            Some(x) => x as usize / 99,
            None => 1024usize,
        };

        if !quiet_mode {
            match content_len {
                Some(cl) => println!("content-length: {}", cl),
                None => println!("content-length missing"),
            };

            match content_type {
                Some(ct) => println!("content-type: {}", ct),
                None => println!("content-type missing"),
            };
        }

        println!("saving to: {}", filename);

        let bar = create_progress_bar(quiet_mode, filename, content_len);

        let mut buf = Vec::new();
        let mut buffer = vec![0; chunk_size];

        loop {
            let bytes_count = resp.read(&mut buffer)?;

            buffer.truncate(bytes_count);

            if !buffer.is_empty() {
                buf.extend(buffer.clone().into_boxed_slice().into_vec().iter().cloned());
                bar.inc(buffer.len() as u64);
            } else {
                break;
            }
        }

        bar.finish();

        // TODO: save to file
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        match file.write_all(&buf) {
            Ok(_) => println!("file saved successfully"),
            Err(e) => eprintln!("failed to write to file: {}", e),
        };
    }

    Ok(())
}
