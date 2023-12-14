use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub(crate) async fn count_lines(
    path: impl AsRef<Path> + Send + 'static,
) -> Result<usize, std::io::Error> {
    tokio::task::spawn_blocking(move || {
        let file = File::open(path)?;
        let line_count = BufReader::new(file).lines().count();
        Ok(line_count)
    })
    .await?
}
