use crate::db;
use crate::api_error::ApiError;
use crate::{schema::staff::{self, dsl::*}, schema::staff_hours::{self, dsl::*}};
use chrono::{NaiveTime};
use crate::diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StaffId {
    pub staff_id: i32
}

#[derive(Deserialize)]
pub struct StaffHourId {
    pub staff_hour_id: i32
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

impl Staff {
    pub fn find_all() -> QueryResult<Vec<Self>> {
        let conn = db::establish_connection();
        staff.order(staff::staff_id.asc()).load::<Self>(&conn)
    }

    pub fn find(id: i32) -> QueryResult<Self> {
        let conn = db::establish_connection();

        staff.filter(staff::staff_id.eq(id)).first::<Self>(&conn)
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
}