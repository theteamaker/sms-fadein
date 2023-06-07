use std::process::{Command};
use std::env::{ args, current_exe };
use serde_json::{Value};
use std::fs::read_to_string;

fn main() {
    let fadein_path = current_exe().unwrap();
    let fadein_parent = fadein_path.parent().unwrap().to_str().unwrap();
    let args: Vec<String> = args().collect();

    let configuration_json: Value = serde_json::from_str(
    read_to_string("./config.json")
        .expect("Could not read the configuration json.")
        .as_str()
    ).unwrap();

    let mut arguments_string = "".to_string();
    let arguments: Vec<(String, Value)> = configuration_json["args"].as_object().unwrap().iter().map(|x| (x.0.to_owned(), x.1.to_owned())).collect();

    for i in arguments {
        arguments_string.push_str(format!("{}={}:", i.0, i.1).as_str());
    }

    arguments_string = arguments_string
        .replace("\\", "")
        .replace("\"", "")
        .strip_suffix(":").unwrap().to_string();

    let filename_arg = match args.len() {
        1 => args[0].as_str(),
        _ => args[1].as_str()
    };

    let _command = Command::new("./ffmpeg")
        .args([
            "-loop", "1",
            "-i", format!("{}/thumbnail.png", fadein_parent).as_str(),
            "-i", filename_arg,
            "-map", "0",
            "-map", "1:a",
            "-vf", format!("fps=30,format=yuv420p,fade={}", arguments_string.as_str()).as_str(),
            "-c:v", "libx264",
            "-preset", "ultrafast",
            "-tune", "stillimage",
            "-framerate", "1",
            "-c:a", "aac",
            "-b:a", "320k",
            "-shortest",
            "-y",
            format!("{}/output.mp4", fadein_parent).as_str()
        ])
        .spawn()
        .unwrap();
    // let process = Command::new("ffmpeg")
    //     .stdin(Stdio::piped())
    //     .spawn();
}