mod gpio;
mod morse;

extern crate dothyphen;
extern crate futures;
extern crate rppal;
extern crate tokio;
extern crate tokio_stream;
extern crate tokio_util;

use crate::morse::MorseManager;
use futures::{AsyncBufReadExt, StreamExt, TryFutureExt, TryStreamExt};
use std::error::Error;
use tokio::task::JoinHandle;
use tokio::{io, join};
use tokio_util::codec::{FramedRead, LinesCodec};

fn dynamic(error: impl Error + 'static) -> Box<dyn Error> {
    Box::new(error)
}

async fn encode() -> JoinHandle<()> {
    let (sender, receiver) = tokio::sync::mpsc::channel::<String>(32);

    let proc = tokio::spawn(async move { MorseManager::new(receiver).process().await.unwrap() });

    let stdin = io::stdin();
    FramedRead::new(stdin, LinesCodec::new())
        .map_err(dynamic)
        .try_for_each(|line| sender.send(line).map_err(dynamic))
        .await
        .unwrap();

    proc
}

#[tokio::main]
async fn main() {
    join!(encode().await).0.unwrap();
}
