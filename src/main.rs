use crate::redis::RedisManager;
use crate::replay::{ReplayItem, ReplayManager};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use tokio_stream::StreamExt;
use tracing::{debug, warn};

mod cli;
mod logging;
mod redis;
mod replay;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::init();
    logging::init();

    let mut replay = ReplayManager::open(cli::input()).await?;
    let pb = ProgressBar::new(*replay.total_count() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ETA {eta}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    let mut redis = RedisManager::connect(cli::redis()).await?;

    while let Some(Ok(item)) = replay.next().await {
        match item {
            ReplayItem::Command(x) => {
                debug!("Replaying command {:?}", x);
                let command = x.get().await;
                if let Err(err) = redis.apply(command).await {
                    warn!("Error while command execution {:?}", err)
                }
            }
            ReplayItem::Empty => {}
        }
        pb.inc(1);
    }

    pb.finish_with_message("Completed");

    Ok(())
}
