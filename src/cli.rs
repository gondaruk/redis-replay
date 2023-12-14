use anyhow::Context;
use clap;
use lazy_static::lazy_static;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
struct Manager<'a> {
    clap_matches: clap::ArgMatches<'a>,
}

lazy_static! {
    static ref MANAGER: Arc<Manager<'static>> = Arc::new(Manager::new());
}

impl Manager<'_> {
    fn new() -> Self {
        Self {
            clap_matches: get_clap_matches(),
        }
    }
}

pub fn init() {
    MANAGER.as_ref();
}

pub fn is_verbose() -> bool {
    return MANAGER.as_ref().clap_matches.is_present("verbose");
}

pub fn is_silent() -> bool {
    return MANAGER.as_ref().clap_matches.is_present("silent");
}

pub fn input() -> &'static Path {
    return Path::new(
        MANAGER
            .as_ref()
            .clap_matches
            .value_of("input")
            .context("input file is required")
            .unwrap(),
    );
}

pub fn redis() -> &'static str {
    return MANAGER
        .as_ref()
        .clap_matches
        .value_of("redis")
        .context("redis connection string is required")
        .unwrap();
}

fn get_clap_matches<'a>() -> clap::ArgMatches<'a> {
    let version = format!("{}", option_env!("VERSION").unwrap_or(""));

    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(version.as_str())
        .about("Replay redis commands recorded using MONITOR command.")
        .author("\n")
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Path to to file with MONITOR output")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("redis")
                .short("r")
                .long("redis")
                .help("Redis connection string, e.g. redis://127.0.0.1:6379")
                .takes_value(true)
                .default_value("redis://127.0.0.1:6379"),
        )
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Be verbose (log debug)")
                .takes_value(false),
        )
        .arg(
            clap::Arg::with_name("silent")
                .short("s")
                .long("silent")
                .help("Be silent (log only errors)")
                .takes_value(false),
        );

    return matches.get_matches();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_arguments() {
        assert_eq!(is_verbose(), false);
        assert_eq!(is_silent(), false);
        assert_eq!(redis(), "redis://127.0.0.1:6379");
        assert_eq!(input(), Path::new(""))
    }
}
