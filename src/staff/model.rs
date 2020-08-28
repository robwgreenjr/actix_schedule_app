use crate::db;
use crate::api_error::ApiError;
use crate::{
    schema::staff::{self, dsl::*}
};
use crate::diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StaffId {
    pub staff_id: i32
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

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
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

impl Staff {
    pub fn find_all() -> QueryResult<Vec<Self>> {
        let conn = db::establish_connection();
        staff.order(staff_id.asc()).load::<Self>(&conn)
    }

    pub fn find(id: i32) -> QueryResult<Self> {
        let conn = db::establish_connection();

        staff.filter(staff::staff_id.eq(id)).first::<Self>(&conn)
    }

    pub fn create(staff_create: StaffCreate) -> Result<Self, ApiError> {
        let conn = db::establish_connection();

        let staff_created = diesel::insert_into(staff::table).values(staff_create).get_result(&conn)?;

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

    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let conn = db::establish_connection();

        let res = diesel::delete(
                staff::table
                    .filter(staff::staff_id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}