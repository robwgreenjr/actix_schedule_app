use crate::api_error::ApiError;
use crate::service::{Service, ServiceId, ServiceCreate, GenerateService, UpdateServiceAll};
use actix_web::{get, post, put, delete, web, HttpResponse};
use serde_json::json;

#[get("/service")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let service = Service::find_all()?;
    Ok(HttpResponse::Ok().json(service))
}

#[get("/service/{service_id}")]
async fn find(id: web::Path<ServiceId>) -> Result<HttpResponse, ApiError> {
    let service = Service::find(id.service_id)?;
    Ok(HttpResponse::Ok().json(service))
}

#[post("/service")]
async fn create(service: web::Json<GenerateService>) -> Result<HttpResponse, ApiError> {
    let service = Service::create(service.into_inner())?;
    Ok(HttpResponse::Ok().json(service))
}

#[put("/service/{service_id}")]
async fn update(id: web::Path<ServiceId>, service: web::Json<ServiceCreate>) -> Result<HttpResponse, ApiError> {
    let service = Service::update(id.service_id, service.into_inner())?;
    Ok(HttpResponse::Ok().json(service))
}

#[put("/full_service/{service_id}")]
async fn update_all(id: web::Path<ServiceId>, service: web::Json<UpdateServiceAll>) -> Result<HttpResponse, ApiError> {
    let service = Service::update_all(id.service_id, service.into_inner())?;
    Ok(HttpResponse::Ok().json(service))
}

#[delete("/service/{service_id}")]
async fn delete(id: web::Path<ServiceId>) -> Result<HttpResponse, ApiError> {
    let service_deleted = Service::delete(id.service_id)?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": service_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(update_all);
    cfg.service(delete);
}