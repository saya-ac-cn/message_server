use serde::{Deserialize, Serialize};

/// 微信公众号模板消息(字段)数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeChatTemplateFieldMessageDTO {
    /// 消息字段内容
    pub value: Option<String>,
    /// 消息字段内容颜色
    pub color: Option<String>
}