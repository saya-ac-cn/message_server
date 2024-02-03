use crate::config::CONTEXT;
use crate::util;
use crate::util::date_time::{DateTimeUtil, DateUtils};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::ops::Add;
use std::string::String;
use serde_json::{Map, Value};
pub struct MailApi {}

impl MailApi {
    /// 发送数据库备份通知邮件
    pub async fn send_dump_massage(mail_to: &str,arg: &Map<String,Value>) -> bool{
        let archive_date_op = arg.get("archive_date");
        if archive_date_op.is_none() {
            log::error!("Could not get archive_date!");
            return false;
        }
        let archive_date = archive_date_op.unwrap().as_str();
        if archive_date.is_none() {
            log::error!("archive_date don't null");
            return false;
        }

        let start_date_op = arg.get("start_date");
        if start_date_op.is_none() {
            log::error!("Could not get start_date!");
            return false;
        }
        let start_date = start_date_op.unwrap().as_str();
        if start_date.is_none() {
            log::error!("start_date don't null");
            return false;
        }

        let end_date_op = arg.get("end_date");
        if end_date_op.is_none() {
            log::error!("Could not get end_date!");
            return false;
        }
        let end_date = end_date_op.unwrap().as_str();
        if end_date.is_none() {
            log::error!("end_date don't null");
            return false;
        }


        let html_template = r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>亲亲里-邮件系统</title>
                <meta http-equiv="Content-Type" content="text/html; charset=UTF-8"/>
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link rel="icon" href="https://saya.ac.cn/favicon.svg"/>
            </head>
            <body>
                <div style="width:available;display: flex;justify-content: center;align-items: center;">
                    <div style="width: 700px;">
                        <div style="height: 100px;display: flex;justify-content: flex-start;align-items: flex-end;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            管理员，您好：
                        </div>
                        <div style="display: flex;justify-content: flex-start;flex-direction: column;padding-bottom: 15px;padding-top: 15px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            <div style="text-indent:30px; margin-bottom: 20px;">
                                平台已于稍早时刻，启动数据库备份计划，下面是执行结果报告：
                            </div>
                            <div style="height: 30px;text-indent:30px">
                                所属日期：${archive_date}
                            </div>
                            <div style="height: 30px;text-indent:30px">
                                开始时间：${start_date}
                            </div>
                            <div style="height: 30px;text-indent:30px">
                                结束时间：${end_date}
                            </div>
                            <div style="height: 30px;text-indent:30px">
                                执行结果：成功
                            </div>
                        </div>
                        <div style="height: 30px;text-indent:30px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            如果您看过上述信息，请忽略此电子邮件。
                        </div>
                        <div style=" height: 30px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            此致！
                        </div>
                        <div style="height: 60px;display: flex;flex-direction: column;align-items: flex-end;justify-content: center;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            <div style="width: 200px;display: flex;flex-direction: column;align-items: center;justify-content: center;">
                                <div>亲亲里·运营中心</div>
                                <div>${send_date}</div>
                            </div>
                        </div>
                        <div style="height: 150px;display: flex;flex-direction: column;align-items: center;justify-content: center;font-family: 'Geneva', Helvetica, Arial, sans-serif;font-smooth: always;-webkit-font-smoothing: antialiased;font-size: 14px;color: #888;">
                            Copyright &copy; <script>document.write(new Date().getFullYear())</script> saya.ac.cn, 亲亲里 All Rights Reserved
                        </div>
                    </div>
                </div>
            </body>
            </html>"#;
        let now = DateTimeUtil::naive_date_time_to_str(
            &Some(DateUtils::now()),
            &util::FORMAT_Y_M_D_H_M_S,
        );
        let html = html_template
            .replace("${send_date}", now.unwrap().as_str())
            .replace("${archive_date}", &archive_date.unwrap().to_string())
            .replace("${start_date}", &start_date.unwrap().to_string())
            .replace("${end_date}", &end_date.unwrap().to_string());

        let mut email_builder = Message::builder();

        let email_from = format!("亲亲里 <{}>", &CONTEXT.config.from_mail);
        // 发件人
        email_builder = email_builder.from(email_from.parse().unwrap());
        // 收件人
        email_builder = email_builder.to(format!("管理员 <{}>", mail_to).parse().unwrap());
        // 主题
        email_builder = email_builder.subject("【亲亲里】应用通知");
        // 邮件内容
        let email_message = email_builder
            .multipart(
                MultiPart::alternative() // This is composed of two parts.
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(String::from(html)),
                    ),
            )
            .unwrap();

        // 邮件服务器账号：
        let creds = Credentials::new(
            String::from(&CONTEXT.config.from_mail),
            String::from(&CONTEXT.config.mail_token),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&CONTEXT.config.mail_server)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email_message) {
            Ok(_) => {
                log::info!("Email sent successfully!");
                true
            },
            Err(e) =>{
                log::error!("Could not send email: {:?}", e);
                false
            }
        }
    }

    /// 发送计划提醒邮件
    pub async fn send_plan_massage(mail: &str,arg: &Map<String,Value>) -> bool{

        let user_op = arg.get("user");
        if user_op.is_none() {
            log::error!("Could not get user name!");
            return false;
        }
        let user = user_op.unwrap().as_str();
        if user.is_none() {
            log::error!("user name don't null");
            return false;
        }
        let flag_op = arg.get("flag");
        if flag_op.is_none() {
            log::error!("Could not get mail flag!");
            return false;
        }
        let flag_op = flag_op.unwrap().as_bool();
        if flag_op.is_none(){
            log::error!("user name don't null");
            return false;
        }
        let flag = flag_op.unwrap();
        let contents_op = arg.get("contents");
        if contents_op.is_none() {
            log::error!("Could not get contents!");
            return false;
        }
        let contents_op = contents_op.unwrap().as_array();
        if contents_op.is_none() {
            log::error!("contents name don't null");
            return false;
        }
        let contents = contents_op.unwrap();
        if contents.is_empty() || 0 == contents.len() {
            log::error!("contents name don't null");
            return false;
        }

        let html_template = r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>亲亲里-邮件系统</title>
                <meta http-equiv="Content-Type" content="text/html; charset=UTF-8"/>
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link rel="icon" href="https://saya.ac.cn/favicon.svg"/>
            </head>
            <body>
                <div style="width:available;display: flex;justify-content: center;align-items: center;">
                    <div style="width: 700px;">
                        <div style="height: 100px;display: flex;justify-content: flex-start;align-items: flex-end;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            ${plan_user}，您好：
                        </div>
                        <div style="display: flex;justify-content: flex-start;flex-direction: column;padding-bottom: 15px;padding-top: 15px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            <div style="text-indent:30px; margin-bottom: 20px;">
                                ${plan_title}
                            </div>
                            ${plan_content}
                        </div>
                        <div style="height: 30px;text-indent:30px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            如果您看过上述信息，请忽略此电子邮件。
                        </div>
                        <div style=" height: 30px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            此致！
                        </div>
                        <div style="height: 60px;display: flex;flex-direction: column;align-items: flex-end;justify-content: center;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                            <div style="width: 200px;display: flex;flex-direction: column;align-items: center;justify-content: center;">
                                <div>亲亲里·运营中心</div>
                                <div>${send_date}</div>
                            </div>
                        </div>
                        <div style="height: 150px;display: flex;flex-direction: column;align-items: center;justify-content: center;font-family: 'Geneva', Helvetica, Arial, sans-serif;font-smooth: always;-webkit-font-smoothing: antialiased;font-size: 14px;color: #888;">
                            Copyright &copy; <script>document.write(new Date().getFullYear())</script> saya.ac.cn, 亲亲里 保留所有权利。
                        </div>
                    </div>
                </div>
            </body>
            </html>"#;

        // 拼凑提醒内容 默认 执行超期未完成的提醒
        let mut title = "以下是您截止昨天还未完成的计划安排，请根据您的情况，合理安排：";
        if flag {
            // 执行正常的提醒
            title = "以下是您今天的计划安排，请根据您的情况，合理安排：";
        }

        // 拼凑提醒内容
        let mut content = String::new();
        let mut index: i32 = 1;
        for item in contents {
            content = content.add(
                format!(
                    "<div style=\"height: 30px;text-indent:30px\">{}、{}</div>",
                    index, item.as_str().unwrap().to_string()
                )
                .as_str(),
            );
            index = index + 1;
        }
        let now = DateTimeUtil::naive_date_time_to_str(
            &Some(DateUtils::now()),
            &util::FORMAT_Y_M_D_H_M_S,
        );
        let html = html_template
            .replace("${send_date}", now.unwrap().as_str())
            .replace("${plan_user}", &user.clone().unwrap().to_string())
            .replace("${plan_content}", content.as_str())
            .replace("${plan_title}", title);
        // 准备收发件人
        let email_from = format!("亲亲里 <{}>", &CONTEXT.config.from_mail);
        let to_mail = format!("{} <{}>", user.unwrap().to_string(), mail);

        let email_builder = Message::builder()
            // 发件人
            .from(email_from.clone().parse().unwrap())
            // 收件人
            .to(to_mail.parse().unwrap())
            // 主题
            .subject("【亲亲里】提醒事项");
        // 邮件内容
        let email_message = email_builder
            .multipart(
                MultiPart::alternative().singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(html)),
                ),
            )
            .unwrap();

        // 邮件服务器账号：
        let creds = Credentials::new(
            String::from(&CONTEXT.config.from_mail),
            String::from(&CONTEXT.config.mail_token),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&CONTEXT.config.mail_server)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email_message) {
            Ok(_) => {
                log::info!("Email sent successfully!");
                true
            },
            Err(e) =>{
                log::error!("Could not send email: {:?}", e);
                false
            }
        }
    }

    pub fn send_example() {
        let email = Message::builder()
            // 发件人
            .from("亲亲里 <504804540@qq.com>".parse().unwrap())
            // 收件人
            .to("管理员 <saya@saya.ac.cn>".parse().unwrap())
            .to("管理员 <228476495@qq.com>".parse().unwrap())
            // 主题
            .subject("【亲亲里】系统通知")
            // 邮件内容
            .body(String::from("Be happy!"))
            .unwrap();

        // 邮件服务器账号：
        let creds = Credentials::new("504804540@qq.com".to_string(), "--------------".to_string());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.qq.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => log::debug!("Email sent successfully!"),
            Err(e) => log::error!("Could not send email: {:?}", e),
        }
    }
}
