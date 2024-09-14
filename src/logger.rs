use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

use std::fmt::Write;

pub async fn highlight_status(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let path = req.path().to_string();
    let method = req.method().to_string();
    let res = next.call(req).await?;
    let status = res.status();

    // Prepare the log message with color
    let mut log_msg = String::new();

    if status.is_success() {
        // Green color for 200 OK
        write!(log_msg, "\x1b[32m").unwrap(); // ANSI for green
    } else {
        write!(log_msg, "\x1b[31m").unwrap(); // ANSI for red
    }
    write!(log_msg, "{}\x1b[0m {} {}", status.as_str(), path, method).unwrap(); // Reset color

    log::info!("{}", log_msg);

    Ok(res)
}
