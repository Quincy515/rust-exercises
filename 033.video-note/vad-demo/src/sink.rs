use crate::vad;
use dasp::Frame;
use hound::{WavSpec, WavWriter};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

pub struct Sink {
    output_dir: PathBuf,
    clip_number: usize,
    prefix: String,
    spec: WavSpec,
    writer: Option<WavWriter<BufWriter<File>>>,
}

impl Sink {
    pub fn new(output_dir: PathBuf, prefix: String, spec: WavSpec) -> Self {
        Sink {
            output_dir,
            prefix,
            spec,
            clip_number: 0,
            writer: None,
        }
    }

    fn get_writer(&mut self) -> &mut WavWriter<BufWriter<File>> {
        if self.writer.is_none() {
            // Lazily initialize the writer. This lets us drop the writer when
            // sent an end_of_transmission and have it automatically start
            // writing to a new clip when necessary.
            let filename = self
                .output_dir
                .join(format!("{}{}.wav", self.prefix, self.clip_number));
            self.clip_number += 1;
            self.writer = Some(WavWriter::create(filename, self.spec).unwrap());
        }

        self.writer.as_mut().unwrap()
    }
}

impl<F> vad::Sink<F> for Sink
where
    F: Frame,
    F::Sample: hound::Sample,
{
    fn record(&mut self, frame: F) {
        let writer = self.get_writer();

        // write all the channels as interlaced audio
        for channel in frame.channels() {
            writer.write_sample(channel).unwrap();
        }
    }

    fn end_of_transmission(&mut self) {
        // if we were previously recording a transmission, remove the writer
        // and let it flush to disk
        if let Some(writer) = self.writer.take() {
            writer.finalize().unwrap();
        }
    }
}
