use lazy_static::lazy_static;
use rocket::tokio::sync::RwLock;

use crate::model::notification::Notification;

lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;

impl NotificationRepository {}
