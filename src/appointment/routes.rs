use crate::api_error::ApiError;
use actix_web::{get, post, put, delete, web, HttpResponse};
use serde_json::json;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(find_all);
    // cfg.service(find);
    // cfg.service(create);
    // cfg.service(update);
    // cfg.service(delete);
}