use lambda_runtime::Context;
use log::{error, info};
use serde::Serialize;

use crate::errors::errors::Error;

#[derive(Serialize)]
struct CloudWatchLog<'a> {
    request_id: &'a str,
    message: &'a str,
    error: Option<&'a str>,
}

pub fn ok(ctx: &Context, msg: &str) {
    let log_message = CloudWatchLog {
        request_id: ctx.request_id.as_str(),
        message: msg,
        error: None,
    };

    if let Ok(log_json) = serde_json::to_string(&log_message) {
        info!("{}", log_json);
    }
}

pub fn error(ctx: &Context, err: &Error) {
    let error_message = err.to_string();

    let log_message = CloudWatchLog {
        request_id: ctx.request_id.as_str(),
        message: "Error",
        error: Some(&error_message),
    };

    if let Ok(log_json) = serde_json::to_string(&log_message) {
        error!("{}", log_json);
    }
}
