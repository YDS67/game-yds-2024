use std::io::BufReader;
use std::sync::mpsc::Receiver;
use rodio::Source;
use miniquad::*;

use crate::settings::FT_DESIRED;
//rx.recv_timeout(std::time::Duration::from_secs_f64(FT_DESIRED))

pub fn playback(rx: &Receiver<bool>) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("assets/music.wav").unwrap();
    let buffer = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();

    let mut play = true;
    let mut time_start = date::now();
    let mut time_curr;
    let mut iter: i32 = 0;

    sink.append(buffer.clone());

    loop {
        iter += 1;
        let play_request = rx.recv();
        match play_request {
            Ok(play_result) => {
                play = play_result;
                println!("Request sent at loop {}", iter)
            },
            Err(_) => {
            },
        };
        if play {
            sink.play()
        } else {
            sink.pause()
        }
        std::thread::sleep(std::time::Duration::from_secs_f64(FT_DESIRED));

        time_curr = date::now();
        if time_curr - time_start > 52.0 {
            sink.append(buffer.clone());
            time_start = date::now()
        }
    }
    
}