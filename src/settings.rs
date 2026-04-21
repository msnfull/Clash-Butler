use config::Config;
use config::ConfigError;
use config::File;
use serde::Deserialize;

use crate::clash::DelayTestConfig;
use crate::speedtest::SpeedTestConfig;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Settings {
    pub fast_mode: bool,
    pub subs: Vec<String>,
    pub rename_node: bool,
    pub rename_pattern: String,
    pub need_add_pool: bool,
    pub test_group_size: usize,
    pub pools: Vec<String>,
    pub connect_test: DelayTestConfig,
    pub speed_test: SpeedTestConfig,
}

impl Settings {
    // 修改这里：增加 config_path 参数
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let settings = Config::builder()
            // 修改这里：使用传入的变量替换硬编码的字符串
            .add_source(File::with_name(config_path))
            .build()?;
        settings.try_deserialize::<Settings>()
    }
}
