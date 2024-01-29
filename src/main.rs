// This script will optimize png files in given directory and all subdirs, if a file was created
// less then 24 hours

use std::env;

use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};

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

fn main() {
    init_logs();
}
