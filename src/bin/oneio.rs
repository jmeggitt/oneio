use std::io::BufRead;
use std::path::PathBuf;
use clap::Parser;
use std::io::Write;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// file to open, remote or local
    #[clap(name = "FILE", parse(from_os_str))]
    file: PathBuf,

    /// read through file and only print out stats
    #[clap(short, long)]
    stats: bool,
}

fn main() {
    let cli = Cli::parse();
    let path: &str = cli.file.to_str().unwrap();
    let reader = match oneio::get_reader(path) {
        Ok(reader) => {reader}
        Err(e) => {
            eprintln!("cannot open {}: {}", path, e.to_string());
            return
        }
    };

    let mut stdout = std::io::stdout();

    let mut count_lines = 0;
    let mut count_chars = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => {l}
            Err(e) => {
                eprintln!("cannot read line from {}: {}", path, e.to_string());
                return;
            }
        };
        if !cli.stats {
            if let Err(e) = writeln!(stdout, "{}", line) {
                if e.kind() != std::io::ErrorKind::BrokenPipe {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
                std::process::exit(0);
            }
        }
        count_chars += line.chars().count();
        count_lines += 1;
    }

    if cli.stats {
        println!("lines: \t {}", count_lines);
        println!("chars: \t {}", count_chars);
    }
}