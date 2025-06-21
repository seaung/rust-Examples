//use std::{fs, sync::OnceLock};
//use config::Config;
//
//static CFG: OnceLock<Config> = OnceLock::new(); // 单例模式只初始化一次
//
//fn init_config(path: &String) {
//    let config_path = fs::canonicalize(path).unwrap_or_else(|e| { panic!("error : {}", e)});
//
//    let cfg = Config::builder()
//        .add_source(config::File::with_name(config_path.to_str().unwrap()))
//        .build()
//        .unwrap();
//
//    let _ = CFG.set(cfg).unwrap(); // 将配置设置进去
//}
//
//fn init_gb() -> &'static Config {
//    CFG.get().expect("get error") // 获取配置
//}
//
//fn main() {
//
//    init_config(&"dev.yml".into());
//    init_gb();
//
//    println!("{:?}", CFG);
//}
//

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct App {
    host: String,
    port: i32,
    mode: String,
}

#[derive(Debug, Clone, Deserialize)]
struct DB {
    host: String,
    user: String,
    pass: String,
    port: i32,
    dbname: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AppConfig {
    app: App,
    db: DB,
}

fn load_config(path: &str) -> Result<AppConfig, config::ConfigError> {

    config::Config::builder()
        .add_source(config::File::with_name(path))
        .build()?
        .try_deserialize()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::fs::canonicalize(&"dev.yml").unwrap_or_else(|e| {panic!("{}", e)});
    let cfg = load_config(path.to_str().unwrap())?;

    println!("{}", cfg.app.host);
    println!("{}", cfg.db.host);

    Ok(())
}
