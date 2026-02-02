use mac_notification_sys::Notification as NF;
use mac_notification_sys::get_bundle_identifier_or_default;
use mac_notification_sys::send_notification;
use mac_notification_sys::set_application;

pub fn init() {
    let bundle = get_bundle_identifier_or_default(env!("CARGO_PKG_NAME"));
    if let Err(e) = set_application(&bundle) {
        tracing::error!("notification set application error: {}", e);
    }
}

pub fn send(subtitle: &str, msg_body: &str) {
    let res = send_notification(
        "Hedwig coming!",
        Some(subtitle),
        msg_body,
        Some(NF::new().sound("Default")),
    );

    if let Err(e) = res {
        tracing::error!("failed to send desktop notification: {}", e);
    }
}
