use log4rs::{Config, config::Appender, append::file::FileAppender, config::Root};
pub fn init_logs() {
    log4rs::init_config(
        Config::builder()
            .appender(
                Appender::builder().build(
                    "file",
                    Box::new(
                        FileAppender::builder()
                            .build(
                            /*    dirs::desktop_dir()
                                    .unwrap()
                                    .join("gta.log")
                                    .to_str()
                                    .unwrap(),
                            )*/
                                "D:/Developments/Rust/ilovegtasa/dll/gta.log",
                            )
                            .unwrap(),
                    ),
                ),
            )
            .build(Root::builder().appender("file").build( log::LevelFilter::Debug))
            .unwrap(),
    ).unwrap();
}