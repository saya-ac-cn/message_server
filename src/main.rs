use actix_web::{App, HttpServer, web};
use message_server::controller::message_controller;
use message_server::config::CONTEXT;
use message_server::middleware::actix_interceptor::ActixInterceptor;
use message_server::config::scheduler::Scheduler;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 日志初始化
    message_server::config::logger::init_log();
    // 数据库连接池初始化
    CONTEXT.init_pool().await;
    Scheduler::init_system_scheduler().await;
    HttpServer::new(|| {
        App::new()
            .wrap(ActixInterceptor {})
            // 登录登出接口单独处理（因为都不在已有的分组中）
            //.route("/backend/login", web::post().to(system_controller::login))
            //.route("/backend/logout", web::post().to(system_controller::logout))
            // 映射静态资源目录
            //.service(fs::Files::new("/warehouse", &CONTEXT.config.data_dir))
            .service(
                web::scope("/message")
                    .service(message_controller::send_wechat_message)
                    .service(message_controller::send_mail_message)
                    // .service(message_controller::user_add)
                    // .service(message_controller::user_update)
                    // .service(message_controller::user_detail)
                    // .service(message_controller::user_remove)
                    // .service(message_controller::user_page)
            )
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}