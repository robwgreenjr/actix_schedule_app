use crate::api_error::ApiError;
use crate::store::{Store, StoreCreate, StoreId, StoreHourId ,StoreHoursCreate, StoreAddressCreate};
use actix_web::{get, post, put, delete, web, HttpResponse};
use serde_json::json;

#[get("/store")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let store = Store::find_all()?;
    Ok(HttpResponse::Ok().json(store))
}

#[get("/store_info/{store_id}")]
async fn find_all_info(id: web::Path<StoreId>) -> Result<HttpResponse, ApiError> {
    let store_info = Store::find_all_data(id.store_id)?;
    Ok(HttpResponse::Ok().json(store_info))
}

#[get("/store_hours")]
async fn find_all_store_hours() -> Result<HttpResponse, ApiError> {
    let store = Store::find_all_store_hours()?;
    Ok(HttpResponse::Ok().json(store))
}

#[get("/store_hours/{store_id}")]
async fn find_store_hours(id: web::Path<StoreId>) -> Result<HttpResponse, ApiError> {
    let store = Store::find_store_hours(id.store_id)?;
    Ok(HttpResponse::Ok().json(store))
}

#[get("/store/{store_id}")]
async fn find(id: web::Path<StoreId>) -> Result<HttpResponse, ApiError> {
    let store = Store::find(id.store_id)?;
    Ok(HttpResponse::Ok().json(store))
}

#[get("/store_address/{store_id}")]
async fn find_address(id: web::Path<StoreId>) -> Result<HttpResponse, ApiError> {
    let store = Store::find_address(id.store_id)?;
    Ok(HttpResponse::Ok().json(store))
}

#[post("/store")]
async fn create(store: web::Json<StoreCreate>) -> Result<HttpResponse, ApiError> {
    let store = Store::create(store.into_inner())?;
    Ok(HttpResponse::Ok().json(store))
}

#[post("/store_address")]
async fn create_address(store_address: web::Json<StoreAddressCreate>) -> Result<HttpResponse, ApiError> {
    let store_address = Store::create_address(store_address.into_inner())?;
    Ok(HttpResponse::Ok().json(store_address))
}

#[put("/store/{store_id}")]
async fn update(id: web::Path<StoreId>, store: web::Json<StoreCreate>) -> Result<HttpResponse, ApiError> {
    let store = Store::update(id.store_id, store.into_inner())?;
    Ok(HttpResponse::Ok().json(store))
}

#[put("/store_address/{store_id}")]
async fn update_address(id: web::Path<StoreId>, store_address: web::Json<StoreAddressCreate>) -> Result<HttpResponse, ApiError> {
    let store_address = Store::update_address(id.store_id, store_address.into_inner())?;
    Ok(HttpResponse::Ok().json(store_address))
}

#[put("/store_hours")]
async fn update_hours(store_hours: web::Json<Vec<StoreHoursCreate>>) -> Result<HttpResponse, ApiError> {
    let store = Store::update_hours(store_hours.into_inner())?;
    Ok(HttpResponse::Ok().json(store))
}

#[put("/store_hours/{store_hour_id}")]
async fn update_one_hour(id: web::Path<StoreHourId>, store_hours: web::Json<StoreHoursCreate>) -> Result<HttpResponse, ApiError> {
    let store = Store::update_one_hour(id.store_hour_id, store_hours.into_inner())?;
    Ok(HttpResponse::Ok().json(store))
}

#[delete("/store/{store_id}")]
async fn delete(id: web::Path<StoreId>) -> Result<HttpResponse, ApiError> {
    let store_deleted = Store::delete(id.store_id)?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": store_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_all_info);
    cfg.service(find_all_store_hours);
    cfg.service(find);
    cfg.service(find_address);
    cfg.service(find_store_hours);
    cfg.service(create);
    cfg.service(create_address);
    cfg.service(update);
    cfg.service(update_one_hour);
    cfg.service(update_hours);
    cfg.service(update_address);
    cfg.service(delete);
}