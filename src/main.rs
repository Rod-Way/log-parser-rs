use chrono::NaiveDateTime;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum Level {
    Info,
    Error,
    Warn,
    Debug,
}

#[derive(Debug)]
struct Filters {
    level: Option<Level>,
    time_range: Duration,
    pattern: Option<String>,
}

#[derive(Debug)]
struct Duration {
    from: Option<NaiveDateTime>,
    to: Option<NaiveDateTime>,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Logs parser with filter and statistics",
    disable_help_flag = true
)]

struct Args {
    /// Input log file
    #[arg(short, long, required = true)]
    input: PathBuf,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Filter by log level
    #[arg(short, long, value_enum)]
    level: Option<Level>,

    /// Search substring in log message
    #[arg(short, long)]
    search: Option<String>,

    /// Filter logs after specified time (format: "YYYY-MM-DD HH:MM:SS")
    #[arg(short('f'), long)]
    from: Option<String>,

    /// Filter logs before specified time
    #[arg(short('t'), long)]
    to: Option<String>,

    /// Show statistics instead of logs
    #[arg(long)]
    stats: bool,

    /// Print help
    #[arg(short('h'), long, action = clap::ArgAction::Help)]
    help: Option<bool>,
}

fn main() {
    let args = Args::parse();

    let mut f: Filters = Filters {
        level: None,
        time_range: Duration {
            from: None,
            to: None,
        },
        pattern: None,
    };

    println!("Указанные аргументы:");
    println!("- input: {:?}", args.input);

    if let Some(output) = &args.output {
        println!("- output: {:?}", output);
    }

    if let Some(level) = args.level {
        // println!("- level: {:?}", level);
        f.level = Some(level);
    }

    if let Some(search) = args.search {
        // println!("- search: {:?}", search);
        f.pattern = Some(search);
    }

    if let Some(from) = &args.from {
        // println!("- from: {:?}", from);
        let from: NaiveDateTime = NaiveDateTime::parse_from_str(from, "%Y-%m-%d %H:%M:%S").unwrap();
        f.time_range.from = Some(from);
    }

    if let Some(to) = &args.to {
        // println!("- to: {:?}", to);
        let to: NaiveDateTime = NaiveDateTime::parse_from_str(to, "%Y-%m-%d %H:%M:%S").unwrap();
        f.time_range.from = Some(to);
    }

    if args.stats {
        println!("- stats");
    }

    println!("{:?}", f);
}
