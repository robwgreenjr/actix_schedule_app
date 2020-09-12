use crate::db;
use crate::api_error::ApiError;
use crate::{
    schema::staff::{self, dsl::*}, 
    schema::staff_hours::{self, dsl::*}, 
    schema::staff_service::{self, dsl::*}, 
    schema::service_variant::{self, dsl::*},
    schema::service::{self, dsl::*},
    schema::block_extra_time::{self, dsl::*}
};
use chrono::{NaiveTime};
use crate::diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::service::model::{
    FullService, BlockExtraTime, ServiceVariant, Service
};

#[derive(Deserialize)]
pub struct StaffId {
    pub staff_id: i32
}

#[derive(Deserialize)]
pub struct StaffHourId {
    pub staff_hour_id: i32
}

#[derive(Deserialize)]
pub struct StaffServiceId {
    pub staff_service_id: i32
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "staff"]
pub struct StaffCreate {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub access: String,
    pub calendar_color: String
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "staff_service"]
pub struct StaffServiceCreate {
    pub staff_id: i32,
    pub service_variant_id: i32,
    pub is_active: i32
}

#[derive(Eq, PartialEq, Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[primary_key(staff_service_id)]
#[table_name = "staff_service"]
pub struct StaffService {
    pub staff_service_id: i32,
    pub staff_id: i32,
    pub service_id: i32,
    pub service_variant_id: i32,
    pub is_active: Option<i32>
}

#[derive(Eq, PartialEq, Identifiable, Serialize, Deserialize, Queryable)]
#[primary_key(staff_id)]
#[table_name = "staff"]
pub struct Staff {
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
    pub phone: Option<String>,
    pub access: Option<String>,
    pub calendar_color: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct BasicStaffInfo {
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub calendar_color: Option<String>
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "staff_hours"]
pub struct StaffHoursCreate {
    pub staff_id: i32,
    pub day_of_week: i32,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>
}

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, AsChangeset, Debug, Copy, Clone)]
#[belongs_to(Staff)]
#[primary_key(staff_hours_id)]
#[table_name = "staff_hours"]
pub struct StaffHours {
    pub staff_hours_id: i32,
    pub staff_id: i32,
    pub day_of_week: i32,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct StaffWithHours {
    pub staff: Staff,
    pub staff_hours: Vec<StaffHours>
}

#[derive(Serialize)]
pub struct StaffWithServices {
    pub staff: Staff,
    pub services: Vec<FullService>
}

impl Staff {
    pub fn find_all() -> QueryResult<Vec<Self>> {
        let conn = db::establish_connection();
        staff.order(staff::staff_id.asc()).load::<Self>(&conn)
    }

    pub fn find(id: i32) -> QueryResult<Self> {
        let conn = db::establish_connection();

        staff.filter(staff::staff_id.eq(id)).first::<Self>(&conn)
    }

    pub fn find_basic(id: i32) -> QueryResult<BasicStaffInfo> {
        let conn = db::establish_connection();

        let staff_data: Staff = staff.filter(staff::staff_id.eq(id)).first::<Self>(&conn)?;

        let staff_member = BasicStaffInfo {
            staff_id: staff_data.staff_id,
            first_name: staff_data.first_name,
            last_name: staff_data.last_name,
            email: staff_data.email,
            phone: staff_data.phone,
            calendar_color: staff_data.calendar_color
        };

        Ok(staff_member)
    }

    pub fn find_all_staff_hours() -> QueryResult<Vec<StaffWithHours>> {
        let conn = db::establish_connection();

        let staff_members = staff::table.load::<Staff>(&conn)?;
        let staff_hours_list = StaffHours::belonging_to(&staff_members)
            .load::<StaffHours>(&conn)?
            .grouped_by(&staff_members);

        let mut staff_final_list: Vec<StaffWithHours> = vec![];

        let mut temp_iter = 0;
        for member in staff_members {
            let current_staff = StaffWithHours {
                staff: member,
                staff_hours: staff_hours_list[temp_iter].clone(),
            };

            staff_final_list.push(current_staff);
            temp_iter += 1;
        }
        
        Ok(staff_final_list)
    }

    pub fn find_staff_hours(id: i32) -> QueryResult<StaffWithHours> {
        let conn = db::establish_connection();

        let staff_member = staff.filter(staff::staff_id.eq(id)).first::<Self>(&conn)?;
        let staff_member_hours = staff_hours
            .filter(staff_hours::staff_id.eq(id))
            .load::<StaffHours>(&conn)?;

        let staff_hour = StaffWithHours {
            staff: staff_member,
            staff_hours: staff_member_hours,
        };

        Ok(staff_hour)
    }

    pub fn create(staff_create: StaffCreate) -> QueryResult<Self> {
        let conn = db::establish_connection();

        let staff_created: Self = diesel::insert_into(staff::table)
            .values(staff_create)
            .get_result(&conn)?;

        // generate staff hours
        let mut hours_list: Vec<StaffHoursCreate> = vec![];

        for x in 0..7 {
            let day_hours = StaffHoursCreate {
                staff_id: staff_created.staff_id,
                day_of_week: x,
                start_time: None,
                end_time: None,
            };

            hours_list.push(day_hours);
        }

        diesel::insert_into(staff_hours::table).values(hours_list).execute(&conn)?;

        Ok(staff_created)
    }

    pub fn update(id: i32, staff_update: StaffCreate) -> Result<Self, ApiError> {
        let conn = db::establish_connection();

        let staff_updated = diesel::update(staff::table)
            .filter(staff::staff_id.eq(id))
            .set(staff_update)
            .get_result(&conn)?;

        Ok(staff_updated)
    }

    pub fn update_one_hour(id: i32, staff_hour: StaffHoursCreate) -> Result<StaffHours, ApiError> {
        let conn = db::establish_connection();

        let staff_hour_updated = diesel::update(staff_hours::table)
            .filter(staff_hours::staff_hours_id.eq(id))
            .set(staff_hour)
            .get_result(&conn)?;

        Ok(staff_hour_updated)
    }

    pub fn update_hours(staff_hours_update: Vec<StaffHoursCreate>) -> Result<(), ApiError> {
        let conn = db::establish_connection();

        for staff_member in staff_hours_update {
            diesel::update(staff_hours::table)
                .filter(staff_hours::staff_id.eq(staff_member.staff_id))
                .filter(staff_hours::day_of_week.eq(staff_member.day_of_week))
                .set(staff_member).execute(&conn);
        }

        Ok(())
    }

    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let conn = db::establish_connection();

        // also make sure to delete other staff data
        diesel::delete(staff_hours::table)
            .filter(staff_hours::staff_id.eq(id))
            .execute(&conn)?;

        let res = diesel::delete(
                staff::table
                    .filter(staff::staff_id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }

    pub fn find_staff_with_service(passed_service_id: i32) -> QueryResult<Vec<StaffService>> {
        let conn = db::establish_connection();

        staff_service.filter(staff_service::service_id.eq(passed_service_id)).load::<StaffService>(&conn)
    }

    pub fn find_service(id: i32) -> QueryResult<StaffWithServices> {
        let conn = db::establish_connection();

        let staff_member = staff.filter(staff::staff_id.eq(id)).first::<Self>(&conn)?;
        let staff_service_join = staff_service
            .filter(staff_service::staff_id.eq(id))
            .load::<StaffService>(&conn)?;

        let mut all_staff_services: Vec<FullService> = vec![];

        for current_service in staff_service_join {
            let current_variant: ServiceVariant = service_variant
                .filter(service_variant::service_variant_id
                .eq(current_service.service_variant_id))
                .first::<ServiceVariant>(&conn)?;
            let current_service: Service = service
                .filter(service::service_id
                .eq(current_variant.service_id))
                .first::<Service>(&conn)?;
            let block_extra: BlockExtraTime = block_extra_time
                .filter(block_extra_time::service_id
                .eq(current_service.service_id))
                .first::<BlockExtraTime>(&conn)?;
            
            let complete_staff_service = FullService {
                service: current_service,
                blocked_time: block_extra,
                variants: vec![current_variant]
            };

            all_staff_services.push(complete_staff_service);
        }

        let final_staff_with_service = StaffWithServices {
            staff: staff_member,
            services: all_staff_services
        };
       

        Ok(final_staff_with_service)
    }

    pub fn add_service(set_staff_id: i32, set_service_id: i32) -> Result<(), ApiError> {
        let conn = db::establish_connection();

        let new_staff_service = StaffServiceCreate {
            staff_id: set_staff_id,
            service_variant_id: set_service_id,
            is_active: 1
        };

        diesel::insert_into(staff_service::table)
            .values(new_staff_service)
            .execute(&conn);

        Ok(())
    }

    pub fn update_staff_services(current_staff_id: i32, updated_services: Vec<StaffServiceCreate>) -> QueryResult<()> {
        let conn = db::establish_connection();

        diesel::delete(staff_service::table
            .filter(staff_service::staff_service_id
            .eq(current_staff_id)))
            .execute(&conn)?;

        for current_staff_service in updated_services {
            diesel::insert_into(staff_service::table)
                .values(current_staff_service)
                .execute(&conn);
        }

        Ok(())
    }

    pub fn delete_service(id: i32)-> Result<usize, ApiError> {
        let conn = db::establish_connection();

        let res = diesel::delete(staff_service::table
            .filter(staff_service::staff_service_id.eq(id)))
            .execute(&conn)?;

        Ok(res)
    }
}