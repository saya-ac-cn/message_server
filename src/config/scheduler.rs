use crate::config::{CONTEXT, SCHEDULER};
use delay_timer::prelude::{Task, TaskBuilder, TaskError};
use log::{error, info};
use crate::util::constant::{WECHAT_ACCESS_TOKEN_PREFIX};
use crate::util::we_chat_api::WeChatApi;

/// 调度任务 https://github.com/BinChengZhao/delay-timer
pub struct Scheduler {}

/// 落实获取token操作
pub async fn execute_take_access_token() {
    WeChatApi::take_access_token().await;
}

/// 构造获取token计划任务
fn build_take_access_token_async_task() -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    task_builder.set_frequency_repeated_by_cron_str("0 15 */2 * * ?").set_task_id(0).set_maximum_running_time(300).spawn_async_routine(execute_take_access_token)
}

/// 落实发送早安的操作
pub async fn execute_morning_token() {
    CONTEXT.user_service.send_care().await;
}

/// 构造发送早安计划任务
fn build_morning_async_task() -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    task_builder.set_frequency_repeated_by_cron_str("0 0 8 * * ?").set_task_id(0).set_maximum_running_time(300).spawn_async_routine(execute_morning_token)
}

impl Scheduler {
    /// 初始化系统级别的调度任务（发生在系统每次启动时）
    pub async fn init_system_scheduler() {
        let scheduler = SCHEDULER.lock().await;
        // 添加获取token的定时任务
        scheduler.add_task(build_take_access_token_async_task().unwrap());
        WeChatApi::take_access_token().await;
        // 添加发送早安的定时任务
        scheduler.add_task(build_morning_async_task().unwrap());
        info!(" - cron pool init finish!");
    }
}