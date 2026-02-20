use rodio::source::{SineWave, Source};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Audio {
    sink: rodio::Sink,
    _stream: rodio::OutputStream,
    beeping: AtomicBool,
}

impl Audio {
    pub fn new() -> Self {
        let (_stream, stream_handle) = rodio::OutputStream::try_default()
            .expect("Failed to open audio stream");
        let sink = rodio::Sink::try_new(&stream_handle)
            .expect("Failed to create sink");
        sink.pause();

        Audio { sink, _stream, beeping: AtomicBool::new(false) }
    }

    pub fn start_beep(&self) {
    if self.sink.empty() {
        self.beeping.store(false, Ordering::Relaxed);
    }

    if !self.beeping.load(Ordering::Relaxed) {
        self.beeping.store(true, Ordering::Relaxed);
        let source = SineWave::new(440.0)
            .take_duration(Duration::from_secs(1))
            .amplify(0.2);
        self.sink.append(source);
        self.sink.play();
    }
}

    pub fn stop_beep(&self) {
        self.sink.pause();
        self.sink.clear();
        self.beeping.store(false, Ordering::Relaxed);
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}