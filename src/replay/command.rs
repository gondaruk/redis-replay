use crate::replay::micros::Micros;

use anyhow::{Context, Result};
use tokio::time::sleep;
use tracing::debug;

pub enum ReplayItem {
    Command(ReplayCommandDelayed),
    Empty,
}

#[derive(Debug)]
pub struct ReplayCommand {
    ts: Micros,
    // NOTE: db is ignored, all commands will be run in a single db
    db: i32,
    command: String,
    args: Vec<String>,
}

impl ReplayCommand {
    pub fn ts(&self) -> &Micros {
        &self.ts
    }
    pub fn db(&self) -> &i32 {
        &self.db
    }
    pub fn command(&self) -> &String {
        &self.command
    }
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn parse(line: String) -> Result<Option<Self>> {
        if line == "OK" {
            // typical indicator of successful MONITOR command, just skip it
            return Ok(None);
        }

        let words = shellwords::split(&line)?;
        let mut iter = words.iter();

        let ts: f64 = iter.next().context("missing ts")?.parse()?;
        let ts = Micros::from_seconds(ts);

        let db: i32 = iter
            .next()
            .context("missing db")?
            .trim_start_matches("[")
            .parse()?;

        let _ = iter.next().context("missing redis uri")?;

        let command: String = iter
            .next()
            .context("missing command")?
            .trim_end_matches("]")
            .to_uppercase()
            .into();

        let args: Vec<String> = iter
            .map(|x| {
                x.replace(r#"\\"#, r#"\"#) // unescape string
                    .replace(r#"\""#, r#"""#) // unescape string
                    .into()
            })
            .collect();

        Ok(Some(Self {
            ts,
            db,
            command,
            args,
        }))
    }

    pub fn clock_diff(&self) -> Micros {
        Micros::since(self.ts)
    }
}

#[derive(Debug)]
pub struct ReplayCommandDelayed {
    command: ReplayCommand,
    at: Micros,
}

impl ReplayCommandDelayed {
    pub fn immediate(command: ReplayCommand) -> Self {
        let at = Micros::now();
        Self { command, at }
    }

    pub fn delayed(command: ReplayCommand, clock_diff: Micros) -> Self {
        let at = *&command.ts + clock_diff;
        Self { command, at }
    }

    pub async fn get(self) -> ReplayCommand {
        if let Some(duration) = self.at.duration_since(Micros::now()) {
            debug!(
                "Sleeping for {:?} before returning delayed command",
                duration
            );
            sleep(duration).await;
        }

        self.command
    }
}
