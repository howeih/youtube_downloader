extern crate reqwest;
extern crate url;
extern crate pbr;

use std::fs::OpenOptions;
use std::str;
use url::Url;
use std::env;
use std::process;
use std::io::{self, Write, Read};
use std::collections::HashMap;
use url::percent_encoding::percent_decode;
use reqwest::header;
use pbr::ProgressBar;

fn parse_video_info(video_info: &str) -> HashMap<&str, &str> {
    let video_info: Vec<&str> = video_info.split("&").collect();
    let mut video_info_map = HashMap::new();
    for info in &video_info {
        let split_info: Vec<&str> = info.split("=").collect();
        video_info_map.insert(split_info[0], split_info[1]);
    }
    video_info_map
}

fn parse_download_url(video_info: String) -> HashMap<String, String> {
    let mut video_mapping: HashMap<String, String> = HashMap::new();
    video_info.split("&").for_each(|x| {
        let video_key: Vec<&str> = x.split("=").collect();
        let value = percent_decode(video_key[1].as_bytes())
            .decode_utf8()
            .unwrap();
        video_mapping.insert(video_key[0].to_string(), value.to_string());
    });
    video_mapping
}

fn get_video_output_name(video_title: &str, video_type: &str) -> String {
    let video_type_ext = video_type
        .split(";")
        .nth(0)
        .unwrap()
        .split("/")
        .nth(1)
        .unwrap();
    format!("{}.{}", video_title, video_type_ext)
}

fn get_user_input(options_len: usize) -> usize {
    let mut input = String::new();
    let input_option: usize;
    let input_range = options_len + 1;
    loop {
        io::stdin().read_line(&mut input).expect(
            "could not read input",
        );
        match input.trim().parse::<usize>() {
            Ok(i) => {
                if i <= input_range && i > 0 {
                    input_option = i;
                    break;
                } else {
                    println!("please enter number between 1 and {}.", input_range);
                    input.clear();
                }
            },
            _ => {
                println!("please enter number between 1 and {}.", input_range);
                input.clear();
            }

        };
    }
    input_option
}

fn show_video_options(stream_map: &Vec<HashMap<String, String>>) {
    let mut options: u32 = 1;
    println!("Which video you want to download ?");
    for i in stream_map {
        let video_quality = i.get("quality").unwrap();
        let video_type = i.get("type").unwrap();
        println!(
            "{}) Quality: {} Type: {}",
            options,
            video_quality,
            video_type
        );
        options += 1;
    }
    println!("{}) Quit", options);
}

fn download_video(video_title: &str, stream_vec: Vec<HashMap<String, String>>) {
    let stream_vec_len = stream_vec.len();
    show_video_options(&stream_vec);
    let user_input = get_user_input(stream_vec_len);
    let input_vec_index = user_input - 1;
    if input_vec_index == stream_vec_len {
        process::exit(1);
    }
    let video_download_url = stream_vec[input_vec_index].get("url").unwrap();
    let video_type = stream_vec[input_vec_index].get("type").unwrap();
    let output_file_name = get_video_output_name(video_title, &video_type);
    let mut resp = reqwest::get(video_download_url).unwrap();
    if resp.status().is_success() {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_file_name)
            .unwrap();
        let len = resp.headers()
            .get::<header::ContentLength>()
            .map(|ct_len| **ct_len)
            .unwrap_or(0);
        println!("Video length: {}", len);
        let mut buf: [u8; 1024] = [0; 1024];
        let mut pb = ProgressBar::new(len);
        loop {
            let byte_read = resp.read(&mut buf).unwrap();
            pb.add(byte_read as u64);
            if byte_read <= 0 {
                pb.finish_print("done");
                break;
            }
            file.write(&buf[0..byte_read]).expect("write file error.");
        }
    }
}

fn parse_stream_map(stream_map: &str) -> Vec<HashMap<String, String>> {
    let decode_stream = percent_decode(stream_map.as_bytes()).decode_utf8().unwrap();
    let stream_options: Vec<&str> = decode_stream.split(",").collect();
    let mut stream_vec: Vec<HashMap<String, String>> = Vec::with_capacity(16);
    for option in stream_options {
        let download_info = parse_download_url(option.to_string());
        stream_vec.push(download_info);
    }
    stream_vec
}

fn main() {
    let video_url = env::args().nth(1).unwrap();
    let parsed_url = Url::parse(&video_url).unwrap();
    let query = parsed_url.query().unwrap();
    let video_id: Vec<&str> = query.split("=").collect();
    let video_info_url = format!(
        "http://www.youtube.com/get_video_info?video_id={0}",
        video_id[1]
    );
    let mut resp = reqwest::get(&video_info_url).unwrap();
    let video_info = resp.text().unwrap();
    let res = str::from_utf8(&video_info.as_bytes()).unwrap();
    let video_info: HashMap<&str, &str> = parse_video_info(res);
    let video_info_title = video_info.get("title").unwrap();
    let video_title = percent_decode(video_info_title.as_bytes())
        .decode_utf8()
        .unwrap();
    let fmt_stream_map = video_info.get("url_encoded_fmt_stream_map").unwrap();
    let stream_vec = parse_stream_map(fmt_stream_map);
    download_video(&video_title, stream_vec);
}
