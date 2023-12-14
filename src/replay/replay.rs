use crate::replay::command::{ReplayCommand, ReplayCommandDelayed, ReplayItem};
use crate::replay::micros::Micros;
use crate::replay::utils::count_lines;
use anyhow::Result;
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio_stream::Stream;
use tracing::{debug, error};

pub struct ReplayManager {
    lines: Lines<BufReader<File>>,
    total_count: usize,
    clock_diff: Option<Micros>,
}

impl ReplayManager {
    pub async fn open(path: impl AsRef<Path> + Clone + Send + 'static) -> Result<Self> {
        let total_count = count_lines(path.clone()).await?;
        let file = File::open(path).await?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let clock_diff = None;
        Ok(Self {
            lines,
            total_count,
            clock_diff,
        })
    }

    pub fn total_count(&self) -> &usize {
        &self.total_count
    }
}

impl Stream for ReplayManager {
    type Item = Result<ReplayItem>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        debug!("poll_next");
        match Pin::new(&mut self.lines).poll_next_line(cx) {
            Poll::Ready(Ok(Some(line))) => {
                debug!("Read line: {}", line);
                match ReplayCommand::parse(line) {
                    Ok(Some(command)) => {
                        debug!("Parsed command: {:?}", command);
                        let clock_diff = self.clock_diff.unwrap_or_else(|| {
                            let clock_diff = command.clock_diff();
                            self.clock_diff = Some(clock_diff);
                            debug!("Clock diff {:?} calculated and stored", clock_diff);
                            clock_diff
                        });
                        let command = ReplayCommandDelayed::delayed(command, clock_diff);

                        Poll::Ready(Some(Ok(ReplayItem::Command(command))))
                    }
                    Ok(None) => {
                        debug!("Line skipped");
                        Poll::Ready(Some(Ok(ReplayItem::Empty)))
                    }
                    Err(err) => {
                        error!("Error parsing command from {:?}", err);
                        Poll::Ready(None)
                    }
                }
            }
            Poll::Ready(Ok(None)) => Poll::Ready(None),
            Poll::Ready(Err(err)) => {
                eprintln!("Error reading file: {}", err);
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
