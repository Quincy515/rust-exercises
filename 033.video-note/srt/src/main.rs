mod srt;
use crate::srt::{CreateSubtitles, Subtitles};
use deepgram::{
    transcription::prerecorded::{
        audio_source::AudioSource,
        options::{Language, Options},
    },
    Deepgram, DeepgramError,
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

static PATH_TO_FILE: &str = "srt/tests/3.mov";
// "tests/examples_prerecorded_from_file_Bueller-Life-moves-pretty-fast.mp3";

#[tokio::main]
async fn main() -> Result<(), DeepgramError> {
    let deepgram_api_key = "9ddaa6dcc9d2744c3741546e3614ea1262675095";
    // env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY environmental variable");

    let dg_client = Deepgram::new(&deepgram_api_key);

    let file = File::open(PATH_TO_FILE).await.unwrap();
    let guess = mime_guess::from_path(&PATH_TO_FILE).first().unwrap();

    let source = AudioSource::from_buffer_with_mime_type(file, guess.to_string());

    let options = Options::builder()
        .punctuate(true)
        .language(Language::zh_CN)
        .utterances(true)
        .build();

    let response = dg_client
        .transcription()
        .prerecorded(source, &options)
        .await?;

    let transcript = &response.results.channels[0].alternatives[0].transcript;

    let mut transcript_file = File::create("srt/dist/transcript.txt").await?;
    transcript_file.write_all(transcript.as_bytes()).await?;

    let subtitles = Subtitles {};
    let srt_string = subtitles.to_srt_zh(response).unwrap();
    println!("{}", srt_string);

    let mut srt_file = File::create("srt/dist/transcript.srt").await?;
    srt_file.write_all(srt_string.as_bytes()).await?;

    Ok(())
}
