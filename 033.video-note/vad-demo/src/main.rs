mod sink;
mod vad;

use crate::sink::Sink;
use anyhow::Result;
use hound::WavReader;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    // open the WAV file
    let reader = WavReader::open("audio.wav")?;
    // we need the header to determine the sample rate
    let header = reader.spec();
    // read all the samples into memory, converting them to a single-channel
    // audio stream
    let samples = reader
        .into_samples::<i16>()
        .map(|result| result.map(|sample| [sample]))
        .collect::<Result<Vec<_>, _>>()?;

    let release_time = (header.sample_rate as f32 * 0.3).round();

    // make sure the output directory exists
    fs::create_dir_all("output")?;
    // initialize our sink
    let mut sink = Sink::new(PathBuf::from("output"), "test".to_string(), header);

    // set up the NoiseGate
    let mut gate = vad::NoiseGate::new(50, release_time as usize);
    // and process all the samples
    gate.process_frames(&samples, &mut sink);

    Ok(())
}
