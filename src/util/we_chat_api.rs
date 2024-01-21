use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use crate::config::CONTEXT;
use crate::util::constant::WECHAT_ACCESS_TOKEN_PREFIX;
use log::{error, info};
use crate::domain::dto::wechat_template_message::WeChatTemplateMessageDTO;

pub struct WeChatApi {}

impl WeChatApi{

    pub async fn take_access_token() -> String {
        // 获取一次redis里面公众号的access_token
        let access_token_op = CONTEXT.redis_client.get_string(&WECHAT_ACCESS_TOKEN_PREFIX).await;
        if access_token_op.is_err(){
            error!("获取access_token缓存异常：{}", access_token_op.unwrap_err());
            return String::from("");
        }
        let access_token = access_token_op.unwrap();
        if !access_token.is_empty() {
            info!("access toke:{}",access_token.clone());
            return access_token;
        }
        // 获取一次access_token
        let grant_type = String::from("client_credential");
        let mut param: HashMap<&str, &String> = HashMap::new();
        param.insert("grant_type", &grant_type);
        param.insert("appid", &CONTEXT.config.wechat_appid);
        param.insert("secret",&CONTEXT.config.wechat_secret);
        let client = reqwest::Client::builder().build().unwrap();
        let send_result = client.get(&CONTEXT.config.wechat_access_token_url).query(&param).send().await;
        if send_result.is_err() {
            error!("调用微信获取access_token接口异常：{}", send_result.unwrap_err());
            return String::from("");
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
            error!("调用微信获取access_token接口返回了异常的数据");
            return String::from("");
        }
        let json = serde_json::from_str(read_result.unwrap().as_str());
        if json.is_err() {
            error!("提取access_token异常：{}", json.unwrap_err());
            return String::from("");
        }
        let data: Value = json.unwrap();
        let code = data.get("errcode");
        if code.is_some() {
            // {"errcode":40013,"errmsg":"invalid appid"}
            let errcode = data["errcode"].as_u64().unwrap();
            let errmsg = data["errmsg"].as_str().unwrap();
            error!("调用微信获取access_token接口异常，错误码：{}，错误原因：{}", errcode,errmsg);
            return String::from("");
        }
        // {"access_token":"ACCESS_TOKEN","expires_in":7200}
        let access_token = data["access_token"].as_str().unwrap();
        let expires_in = data["expires_in"].as_u64().unwrap();
        CONTEXT.redis_client.set_string_ex(&WECHAT_ACCESS_TOKEN_PREFIX,access_token,Some(Duration::from_secs(expires_in))).await;
        info!("new access toke:{}",access_token.clone());
        return format!("{}",access_token);
    }

    pub async fn do_send_wechat_message(data:&WeChatTemplateMessageDTO) -> bool{
        let access_token = WeChatApi::take_access_token().await;
        if access_token.is_empty() {
            error!("发送公众号模板消息前获取access_token异常，终止此次发送");
            return false;
        }
        let mut param: HashMap<&str, &String> = HashMap::new();
        param.insert("access_token",&access_token);
        let client = reqwest::Client::builder().build().unwrap();
        let send_result = client.post(&CONTEXT.config.wechat_send_template_url).query(&param).json(data).send().await;
        if send_result.is_err() {
            error!("调用微信发送公众号模板消息接口异常：{}", send_result.unwrap_err());
            return false;
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
            error!("调用微信发送公众号模板消息接口返回了异常的数据");
            return false;
        }
        let json = serde_json::from_str(read_result.unwrap().as_str());
        if json.is_err() {
            error!("提取公众号发送模板消息接口回执数据异常：{}", json.unwrap_err());
            return false;
        }
        let data: Value = json.unwrap();
        let code = data["errcode"].as_u64().unwrap();
        if 0 != code {
            error!("调用微信发送公众号模板消息接口返回了异常的状态码：{}", code);
            return false;
        }
        return true;
    }

}