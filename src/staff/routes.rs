use crate::api_error::ApiError;
use crate::staff::{Staff, StaffCreate, StaffId, StaffHourId ,StaffHoursCreate};
use actix_web::{get, post, put, delete, web, HttpResponse};
use serde_json::json;

#[get("/staff")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let staff = Staff::find_all()?;
    Ok(HttpResponse::Ok().json(staff))
}

#[get("/staff_hours")]
async fn find_all_staff_hours() -> Result<HttpResponse, ApiError> {
    let staff = Staff::find_all_staff_hours()?;
    Ok(HttpResponse::Ok().json(staff))
}

#[get("/staff_hours/{staff_id}")]
async fn find_staff_hours(id: web::Path<StaffId>) -> Result<HttpResponse, ApiError> {
    let staff = Staff::find_staff_hours(id.staff_id)?;
    Ok(HttpResponse::Ok().json(staff))
}

#[get("/staff/{staff_id}")]
async fn find(id: web::Path<StaffId>) -> Result<HttpResponse, ApiError> {
    let staff = Staff::find(id.staff_id)?;
    Ok(HttpResponse::Ok().json(staff))
}

#[post("/staff")]
async fn create(staff: web::Json<StaffCreate>) -> Result<HttpResponse, ApiError> {
    let staff = Staff::create(staff.into_inner())?;
    Ok(HttpResponse::Ok().json(staff))
}

#[put("/staff/{staff_id}")]
async fn update(id: web::Path<StaffId>, staff: web::Json<StaffCreate>) -> Result<HttpResponse, ApiError> {
    let staff = Staff::update(id.staff_id, staff.into_inner())?;
    Ok(HttpResponse::Ok().json(staff))
}

#[put("/staff_hours")]
async fn update_hours(staff_hours: web::Json<Vec<StaffHoursCreate>>) -> Result<HttpResponse, ApiError> {
    let staff = Staff::update_hours(staff_hours.into_inner())?;
    Ok(HttpResponse::Ok().json(staff))
}

#[put("/staff_hours/{staff_hour_id}")]
async fn update_one_hour(id: web::Path<StaffHourId>, staff_hours: web::Json<StaffHoursCreate>) -> Result<HttpResponse, ApiError> {
    let staff = Staff::update_one_hour(id.staff_hour_id, staff_hours.into_inner())?;
    Ok(HttpResponse::Ok().json(staff))
}

#[delete("/staff/{staff_id}")]
async fn delete(id: web::Path<StaffId>) -> Result<HttpResponse, ApiError> {
    let staff_deleted = Staff::delete(id.staff_id)?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": staff_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_all_staff_hours);
    cfg.service(find);
    cfg.service(find_staff_hours);
    cfg.service(create);
    cfg.service(update);
    cfg.service(update_one_hour);
    cfg.service(update_hours);
    cfg.service(delete);
}