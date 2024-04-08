use config::{Config, File};


/// 配置文件 映射后的结构配置
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    /// 当前服务地址
    pub server_url: String,
    /// 主数据库地址
    pub primary_database_url: String,
    /// redis地址
    pub redis_url: String,
    /// 日志目录 "target/logs/"
    pub log_dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    pub log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    pub log_pack_compress: String,
    /// 日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub log_rolling_type: String,
    /// 日志等级
    pub log_level: String,
    pub log_type: String,
    pub log_chan_len: Option<usize>,
    /// 白名单接口
    pub white_list_api: Vec<String>,
    /// 项目产生的数据目录
    pub data_dir: String,
    /// 公众号的appid
    pub wechat_appid: String,
    /// 公众号的secret
    pub wechat_secret: String,
    /// 公众号获取access_token的url
    pub wechat_access_token_url: String,
    /// 公众号发送模板消息的url: String,
    pub wechat_send_template_url: String,
    /// 发件人
    pub from_mail: String,
    /// 邮件服务器地址
    pub mail_server: String,
    /// 邮件服务器发送token
    pub mail_token: String,
    /// 高德地图天气接口地址
    pub amap_url: String,
    ///高德天气接口地址密钥
    pub amap_key: String,
    /// 发送微信消息的模板(早安)
    pub wechat_notice_template: String,
}

impl Default for ApplicationConfig {
    /// 加载yml配置，这里还不能用log::info!进行日志打印，因为还没有初始化
    fn default() -> Self {
        let mut config = Config::default();
        config.merge(File::with_name("application.yml")).unwrap();
        let result: ApplicationConfig = config.try_into().unwrap();
        if result.debug {
            println!("[message_server] load config:{:?}", result);
            println!("[message_server] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[message_server] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}