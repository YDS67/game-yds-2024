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
    let dur = buffer.total_duration().unwrap();

    let mut play = true;
    let mut time_start = date::now();
    let mut time_curr;
    let mut time_paused = 0.0;
    let mut iter: i32 = 0;

    sink.append(buffer.clone());
    let mut play_request = rx.try_recv();

    loop {
        iter += 1;
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
            sink.pause();
            time_paused += FT_DESIRED
        }
        std::thread::sleep(std::time::Duration::from_secs_f64(FT_DESIRED));
        time_curr = date::now();
        if time_curr - time_start - time_paused > dur.as_secs_f64() {
            sink.append(buffer.clone());
            time_start = date::now();
            time_paused = 0.0;
        } else {
            play_request = rx.try_recv();
        }
    }
    
}