use std::env;
use std::time::Duration;

use futures::stream::StreamExt;

use deepgram::{transcription::prerecorded::options::{Language, Options}, Deepgram, DeepgramError};

#[tokio::main]
async fn main() -> Result<(), DeepgramError> {
    let dg = Deepgram::new(env::var("DEEPGRAM_API_KEY").unwrap());

    let options = Options::builder()
        .smart_format(true)
        .language(Language::en_US)
        .build();

    let mut results = dg
        .transcription()
        .stream_request(&options)
        .file(
            env::var("FILENAME").unwrap(),
            3174,
            Duration::from_millis(16),
        )
        .await?
        .start()
        .await?;

    while let Some(result) = results.next().await {
        println!("got: {:?}", result);
    }

    Ok(())
}
