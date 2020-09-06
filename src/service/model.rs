use crate::db;
use crate::api_error::ApiError;
use crate::{schema::service::{self, dsl::*}, schema::service_variant::{self, dsl::*}, schema::block_extra_time::{self, dsl::*}};
use chrono::{NaiveTime};
use crate::diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ServiceId {
    pub service_id: i32
}

#[derive(Deserialize)]
pub struct ServiceVariantId {
    pub staff_variant_id: i32
}

#[derive(Deserialize)]
pub struct BlockExtraTimeId {
    pub block_extra_time: i32
}

#[derive(Serialize)]
pub struct FullService {
    pub service: Service,
    pub blocked_time: BlockExtraTime,
    pub variants: Vec<ServiceVariant>
}

#[derive(Deserialize)]
pub struct GenerateService {
    pub name: String,
    pub description: Option<String>,
    pub is_active: i32,
    pub category: Option<String>,
    pub before_time: Option<NaiveTime>,
    pub after_time: Option<NaiveTime>,
    pub variants: Vec<GenerateServiceVariant>
}

#[derive(Deserialize)]
pub struct GenerateServiceVariant {
    pub price: f64,
    pub duration: Option<NaiveTime>
}

#[derive(Eq, PartialEq, Identifiable, Serialize, Deserialize, Queryable)]
#[primary_key(service_id)]
#[table_name = "service"]
pub struct Service {
    pub service_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_active: i32,
    pub category: Option<String>
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "service"]
pub struct ServiceCreate {
    pub name: String,
    pub description: Option<String>,
    pub is_active: i32,
    pub category: Option<String>
}

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, AsChangeset, Debug, Clone)]
#[belongs_to(Service)]
#[primary_key(service_variant_id)]
#[table_name = "service_variant"]
pub struct ServiceVariant {
    pub service_variant_id: i32,
    pub service_id: i32,
    pub price: f64,
    pub duration: Option<NaiveTime>
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "service_variant"]
pub struct ServiceVariantCreate {
    pub service_id: i32,
    pub price: f64,
    pub duration: Option<NaiveTime>
}

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, AsChangeset, Debug, Copy, Clone)]
#[belongs_to(Service)]
#[primary_key(block_extra_time_id)]
#[table_name = "block_extra_time"]
pub struct BlockExtraTime {
    pub block_extra_time_id: i32,
    pub service_id: i32,
    pub before_time: Option<NaiveTime>,
    pub after_time: Option<NaiveTime>
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "block_extra_time"]
pub struct BlockExtraTimeCreate {
    pub service_id: i32,
    pub before_time: Option<NaiveTime>,
    pub after_time: Option<NaiveTime>
}

#[derive(Deserialize)]
pub struct UpdateServiceAll {
    pub name: String,
    pub description: Option<String>,
    pub is_active: i32,
    pub category: Option<String>,
    pub before_time: Option<NaiveTime>,
    pub after_time: Option<NaiveTime>,
    pub variants: Vec<ServiceVariant>
}

impl Service {
    pub fn find_all() -> QueryResult<Vec<FullService>> {
        let conn = db::establish_connection();

        let all_services = service::table.load::<Service>(&conn)?;
        let all_blocked_time = BlockExtraTime::belonging_to(&all_services)
            .load::<BlockExtraTime>(&conn)?
            .grouped_by(&all_services);
        let all_services_variants = ServiceVariant::belonging_to(&all_services)
            .load::<ServiceVariant>(&conn)?
            .grouped_by(&all_services);

        let mut services_final_list: Vec<FullService> = vec![];
        
        let mut temp_iter = 0;
        for current_service in all_services {
            let current_service = FullService {
                service: current_service,
                blocked_time: all_blocked_time[temp_iter][0].clone(),
                variants: all_services_variants[temp_iter].clone()
            };

            services_final_list.push(current_service);
            temp_iter += 1;
        }


        Ok(services_final_list)
    }

    pub fn find(id: i32) -> QueryResult<FullService> {
        let conn = db::establish_connection();

        let service_entity: Self = service.filter(service::service_id.eq(id)).first::<Self>(&conn)?;
        let block_extra: BlockExtraTime = block_extra_time.filter(block_extra_time::service_id.eq(id)).first::<BlockExtraTime>(&conn)?;
        let service_variants: Vec<ServiceVariant> = service_variant.filter(service_variant::service_id.eq(id)).load::<ServiceVariant>(&conn)?;

        let full_service = FullService {
            service: service_entity,
            blocked_time: block_extra,
            variants: service_variants
        };

        Ok(full_service)
    }

    pub fn create(service_data: GenerateService) -> QueryResult<Self> {
        let conn = db::establish_connection();

        let service_create = ServiceCreate {
            name: service_data.name,
            description: service_data.description,
            is_active: service_data.is_active,
            category: service_data.category
        };

        let service_created: Self = diesel::insert_into(service::table)
            .values(service_create)
            .get_result(&conn)?;

        // set extra blocked time
        let block_extra_time_create = BlockExtraTimeCreate {
            service_id: service_created.service_id,
            before_time: service_data.before_time,
            after_time: service_data.after_time
        };

        diesel::insert_into(block_extra_time::table)
            .values(block_extra_time_create)
            .execute(&conn)?;

        // Create service variants
        for variant in service_data.variants {
            let service_variant_create = ServiceVariantCreate {
                service_id: service_created.service_id,
                price: variant.price,
                duration: variant.duration
            };
    
            diesel::insert_into(service_variant::table)
                .values(service_variant_create)
                .execute(&conn)?;
        }

        Ok(service_created)
    }

    pub fn update(id: i32, service_update: ServiceCreate) -> Result<Self, ApiError> {
        let conn = db::establish_connection();

        let service_updated = diesel::update(service::table)
            .filter(service::service_id.eq(id))
            .set(service_update)
            .get_result(&conn)?;

        Ok(service_updated)
    }

    pub fn update_variant(id: i32, variant_update: ServiceVariantCreate) -> Result<ServiceVariant, ApiError> {
        let conn = db::establish_connection();

        let variant_updated = diesel::update(service_variant::table)
            .filter(service_variant::service_variant_id.eq(id))
            .set(variant_update)
            .get_result(&conn)?;

        Ok(variant_updated)
    }

    pub fn update_time_block(id: i32, block_update: BlockExtraTimeCreate) -> Result<BlockExtraTime, ApiError> {
        let conn = db::establish_connection();

        let block_updated = diesel::update(block_extra_time::table)
            .filter(block_extra_time::service_id.eq(id))
            .set(block_update)
            .get_result(&conn)?;

        Ok(block_updated)
    }

    pub fn update_all(id: i32, service_update: UpdateServiceAll) -> Result<Self, ApiError> {
        let conn = db::establish_connection();

        let service_update_info = ServiceCreate {
            name: service_update.name,
            description: service_update.description,
            is_active: service_update.is_active,
            category: service_update.category
        };

        let service_updated = diesel::update(service::table)
            .filter(service::service_id.eq(id))
            .set(service_update_info)
            .get_result(&conn)?;
        
        for variant in service_update.variants {
            let service_variant_update = ServiceVariantCreate {
                service_id: id,
                price: variant.price,
                duration: variant.duration
            };
            
            diesel::update(service_variant::table)
                .filter(service_variant::service_variant_id.eq(variant.service_variant_id))
                .set(service_variant_update)
                .execute(&conn)?;
        }

        let blocked_time_update = BlockExtraTimeCreate {
            service_id: id,
            before_time: service_update.before_time,
            after_time: service_update.after_time
        };
        
        diesel::update(block_extra_time::table)
            .filter(block_extra_time::service_id.eq(id))
            .set(blocked_time_update)
            .execute(&conn)?;

        Ok(service_updated)
    }

    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let conn = db::establish_connection();

        // also make sure to delete other service data
        diesel::delete(service_variant::table)
            .filter(service_variant::service_id.eq(id))
            .execute(&conn)?;

        diesel::delete(block_extra_time::table)
            .filter(block_extra_time::service_id.eq(id))
            .execute(&conn)?;

        let res = diesel::delete(
                service::table
                    .filter(service::service_id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}
