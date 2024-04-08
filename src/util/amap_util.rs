use log::error;
use serde_json::Value;
use std::collections::HashMap;
use crate::config::CONTEXT;
use crate::domain::dto::weather_info::WeatherInfoDTO;

pub struct AmapUtils {}

impl AmapUtils {
    pub async fn weather_info(city: &String) -> WeatherInfoDTO {
        let mut map: HashMap<&str, &String> = HashMap::new();
        map.insert("key", &CONTEXT.config.amap_key);
        map.insert("city", city);
        let client = reqwest::Client::builder().build().unwrap();

        let send_result = client
            .get(&CONTEXT.config.amap_url)
            .query(&map)
            .send()
            .await;
        if send_result.is_err() {
            error!("天气查询异常：{}", send_result.unwrap_err());
            return WeatherInfoDTO::new();
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
            error!("解码天气异常：{}", read_result.unwrap_err());
            return WeatherInfoDTO::new();
        }
        let json_str = read_result.unwrap();
        let json = serde_json::from_str(json_str.as_str());
        if json.is_err() {
            error!("提取天气异常：{}", json.unwrap_err());
            return WeatherInfoDTO::new();
        }
        let location: Value = json.unwrap();
        let info_code = location["infocode"].as_str().unwrap();
        if "10000" != info_code {
            error!("提取天气异常，错误的状态码：{}", info_code);
            return WeatherInfoDTO::new();
        }
        let lives = location["lives"].as_array().unwrap();
        if lives.is_empty() {
            error!("提取天气异常，没有有效的数据返回");
            return WeatherInfoDTO::new();
        }
        let live = lives.get(0).unwrap();
        let weather_info = WeatherInfoDTO {
            city: Some(String::from(live["city"].as_str().unwrap())),
            weather: Some(String::from(live["weather"].as_str().unwrap())),
            temperature: Some(String::from(live["temperature"].as_str().unwrap())),
            winddirection: Some(String::from(live["winddirection"].as_str().unwrap())),
            windpower: Some(String::from(live["windpower"].as_str().unwrap())),
            humidity: Some(String::from(live["humidity"].as_str().unwrap())),
        };
        return weather_info;
    }
}
