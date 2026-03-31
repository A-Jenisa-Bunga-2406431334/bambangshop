use rocket::serde::json::Json;
use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: &str, subscriber: Json<Subscriber>) -> Result<Json<Subscriber>> {
    return match NotificationService::subscribe(product_type, subscriber.into_inner()) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e)
    };
}

#[post("/unsubscribe/<product_type>/<subscriber_url>")]
pub fn unsubscribe(product_type: &str, subscriber_url: &str) -> Result<Json<Subscriber>> {
    todo!()
}