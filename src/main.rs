// This script will optimize png files in given directory and all subdirs, if a file was created
// less then 24 hours

use std::{
    env, fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;

use glob::glob;
use log::{info, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};

#[derive(Parser, Debug)]
struct CliArgs {
    // path to a target directory
    #[arg(short, long)]
    target_dir: String,
}

fn init_logs() {
    let stdout = ConsoleAppender::builder().build();
    let current_dir = env::current_dir().unwrap();
    let logfile_path = current_dir.join("png-optimizer.log");
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - [{l}] - {m}\n")))
        .build(logfile_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .logger(
            Logger::builder()
                .appender("logfile")
                .additive(false)
                .build("png-optimizer", LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn optimize_images(target_dir: &str) {
    let dt_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let pattern = format!("{}/**/*.png", target_dir);
    let mut image_counter = 0;
    for entry in glob(&pattern).unwrap() {
        let item = match entry {
            Ok(item) => item,
            Err(err) => {
                log::error!("{}", err);
                continue;
            }
        };
        let item_created = match fs::metadata(&item).unwrap().created() {
            Ok(item_created) => item_created.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            Err(_) => continue,
        };
        let time_diff = (dt_now - item_created) / 60 / 60;
        if time_diff > 24 {
            continue;
        }
        image_counter += 1;

        let opt_options = oxipng::Options::default();

        let infile = oxipng::InFile::from(&item);
        let outfile = oxipng::OutFile::from_path(item.clone());

        match oxipng::optimize(&infile, &outfile, &opt_options) {
            Ok(_) => log::info!("optimized: {:?}", &item),
            Err(err) => log::error!("Can't optimize {:?}: {}", &item, err),
        }
    }
    log::info!("Optimized {} images", image_counter);
}

fn main() {
    init_logs();
    let cli_args = CliArgs::parse();
    optimize_images(&cli_args.target_dir);
}
