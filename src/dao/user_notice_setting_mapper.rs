use rbatis::{crud, impl_select};
use crate::domain::entity::UserNoticeSetting;

crud!(UserNoticeSetting {});
pub struct UserNoticeSettingMapper {}