use serde::{Deserialize, Serialize};

/// 高德返回的天气信息
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeatherInfoDTO {
    /// 城市名
    pub city: Option<String>,
    /// 天气现象（汉字描述）
    pub weather: Option<String>,
    /// 实时气温，单位：摄氏度
    pub temperature: Option<String>,
    /// 风向描述
    pub winddirection: Option<String>,
    /// 风力级别，单位：级
    pub windpower: Option<String>,
    ///空气湿度
    pub humidity: Option<String>
}

impl WeatherInfoDTO {
    pub fn new() -> WeatherInfoDTO {
        let weather_info = WeatherInfoDTO {
            city: Some(String::from("-")),
            weather: Some(String::from("-")),
            temperature: Some(String::from("-")),
            winddirection: Some(String::from("-")),
            windpower: Some(String::from("-")),
            humidity: Some(String::from("-")),
        };
        return weather_info;
    }
}