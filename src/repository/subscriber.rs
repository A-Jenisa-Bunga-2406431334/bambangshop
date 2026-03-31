use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        if !SUBSCRIBERS.contains_key(product_type) {
            SUBSCRIBERS.insert(product_type.to_string(), DashMap::new());
        }
        let subscriber_value = subscriber.clone();
        SUBSCRIBERS
            .get(product_type)
            .unwrap()
            .insert(subscriber_value.url.clone(), subscriber_value);
        return subscriber;
    }

    pub fn list_all(product_type: &str) -> Vec<Subscriber> {
        if !SUBSCRIBERS.contains_key(product_type) {
            return Vec::new();
        }
        return SUBSCRIBERS
            .get(product_type)
            .unwrap()
            .iter()
            .map(|f| f.value().clone())
            .collect();
    }

    pub fn delete(product_type: &str, url: &str) -> Option<Subscriber> {
        if !SUBSCRIBERS.contains_key(product_type) {
            return None;
        }
        let result = SUBSCRIBERS
            .get(product_type)
            .unwrap()
            .remove(url);
        if !result.is_none() {
            return Some(result.unwrap().1);
        }
        return None;
    }
}