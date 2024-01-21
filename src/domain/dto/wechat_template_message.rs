use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::domain::dto::wechat_template_field_message::WeChatTemplateFieldMessageDTO;

/// 微信公众号模板消息(整体)数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeChatTemplateMessageDTO {
    /// 接收者openid
    pub touser: Option<String>,
    /// 模板ID
    pub template_id: Option<String>,
    /// 模板跳转链接 若不传则模板无跳转
    pub url: Option<String>,
    /// 标题颜色
    pub topcolor: Option<String>,
    /// 模板正文消息内容
    pub data: Option<HashMap<String, WeChatTemplateFieldMessageDTO>>
}