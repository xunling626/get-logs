use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init_log(loglevel: String) {
    let level = if loglevel.eq("trace") {
        LevelFilter::Trace
    } else if loglevel.eq("debug") {
        LevelFilter::Debug
    } else if loglevel.eq("info") {
        LevelFilter::Info
    } else if loglevel.eq("warn") {
        LevelFilter::Warn
    } else if loglevel.eq("error") {
        LevelFilter::Error
    } else {
        panic!("Error")
    };
    //stdout是控制台输出器 ConsoleAppender这个输出器是直接输出到控制台
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            //修改{}里面的值能有不同的效果
            "[Console] {f}  {d} - {l} -{t} - {m}{n}",
        )))
        .build();
    //logfile是文件输出器 FileAppender使用了这个输出器得给他一个位置输出
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[File] {d} - {l} - {t} - {m}{n}",
        )))
        .build("log/test.log")
        .unwrap();
    //config 是创建一个日志配置对象 然后加了两个日志输出目标一个是输出到控制台 一个是输出到日志文件
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("console")
                .appender("logfile")
                .build(level),
        )
        .unwrap();
    //这一句是将配置应用到log4rs库
    let _ = log4rs::init_config(config).unwrap();
}
