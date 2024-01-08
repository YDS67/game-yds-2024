use std::io::BufReader;

use rodio::Source;

pub fn playback() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("assets/music.wav").unwrap();
    let buffer = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();

    loop {
        sink.append(buffer.clone());
        sink.sleep_until_end();
    }
}