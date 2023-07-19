use std::{
    fs, panic,
    path::{Path, PathBuf},
    time::Instant,
};

use crate::formatter::{format_file, FormatterSettings};
use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use glob::glob;
use rayon::{iter::ParallelIterator, prelude::IntoParallelIterator};
mod formatter;

#[derive(Debug, Parser)]
#[command(name = "hirola-kit")]
#[command(about = "hirola development kit and tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Format hirola html! macro code
    #[command(arg_required_else_help = true)]
    Format(Format),
}

#[derive(Debug, Args)]
struct Format {
    /// A file, directory or glob
    input_pattern: String,

    // Maximum width of each line
    #[arg(short, long)]
    max_width: Option<usize>,

    // Number of spaces per tab
    #[arg(short, long)]
    tab_spaces: Option<usize>,

    // Config file
    #[arg(short, long)]
    config_file: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Format(args) => {
            let settings = create_settings(&args).unwrap();

            // Print settings
            println!("{}", toml::to_string_pretty(&settings).unwrap());

            let is_dir = fs::metadata(&args.input_pattern)
                .map(|meta| meta.is_dir())
                .unwrap_or(false);

            let glob_pattern = if is_dir {
                format!("{}/**/*.rs", &args.input_pattern)
            } else {
                args.input_pattern
            };

            let file_paths: Vec<_> = glob(&glob_pattern)
                .expect("failed to read glob pattern")
                .collect();

            let total_files = file_paths.len();
            let start_formatting = Instant::now();
            file_paths.into_par_iter().for_each(|result| {
                let print_err = |path: &Path, err| {
                    println!("❌ {}", path.display());
                    eprintln!("\t\t{}", err);
                };

                match result {
                    Ok(path) => match format_glob_result(&path, settings) {
                        Ok(_) => println!("✅ {}", path.display()),
                        Err(err) => print_err(&path, &err.to_string()),
                    },
                    Err(err) => print_err(err.path(), &err.error().to_string()),
                };
            });
            let end_formatting = Instant::now();
            println!(
                "Formatted {} files in {} ms",
                total_files,
                (end_formatting - start_formatting).as_millis()
            )
        }
    }
}

fn format_glob_result(file: &PathBuf, settings: FormatterSettings) -> anyhow::Result<()> {
    let formatted = panic::catch_unwind(|| format_file(file, settings))
        .map_err(|e| anyhow::anyhow!(e.downcast::<String>().unwrap()))??;
    fs::write(file, formatted)?;
    Ok(())
}

fn create_settings(args: &Format) -> anyhow::Result<FormatterSettings> {
    let mut settings = args
        .config_file
        .as_ref()
        .map(|path| {
            load_config(path)
                .with_context(|| format!("failed to load config file: {}", path.display()))
        })
        .unwrap_or_else(|| {
            let default_config: PathBuf = "hirola.toml".into();
            if default_config.exists() {
                load_config(&default_config).with_context(|| {
                    format!("failed to load config file: {}", default_config.display())
                })
            } else {
                Ok(FormatterSettings::default())
            }
        })?;

    if let Some(max_width) = args.max_width {
        settings.max_width = max_width;
    }

    if let Some(tab_spaces) = args.tab_spaces {
        settings.tab_spaces = tab_spaces;
    }
    Ok(settings)
}

fn load_config(path: &PathBuf) -> anyhow::Result<FormatterSettings> {
    let config = fs::read_to_string(path).context("could not read config file")?;
    let settings: FormatterSettings =
        toml::from_str(&config).context("could not parse config file")?;

    Ok(settings)
}
