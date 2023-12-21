use crate::futures::StreamExt;
use code::MorseCode;
use futures::TryStreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;

mod code;

const SMALL: Duration = Duration::from_millis(50);
const LARGE: Duration = Duration::from_millis(150);

pub struct MorseManager {
    receiver: Receiver<String>,
}

impl MorseManager {
    pub fn new(receiver: Receiver<String>) -> Self {
        MorseManager { receiver }
    }

    pub async fn process(self) -> Result<(), Box<dyn Error>> {
        ReceiverStream::new(self.receiver)
            .map(|str| dothyphen::translate::to_morse(&str))
            .flat_map(|str| futures::stream::iter(format!("{}/n", str).chars().collect::<Vec<_>>()))
            .filter(|&char| async move {
                if char == ' ' {
                    sleep(SMALL).await;
                }
                char != ' '
            })
            .filter_map(|char| async move { MorseCode::new(char) })
            .then(|morse| async move { Ok((morse.duration(), morse.led().output()?)) })
            .try_for_each(|(duration, mut led)| async move {
                led.set_high();
                sleep(duration).await;
                led.set_low();
                sleep(SMALL).await;
                Ok::<(), Box<dyn Error>>(())
            })
            .await?;
        Ok(())
    }
}
