use crate::db;
use crate::api_error::ApiError;
use crate::{schema::store::{self, dsl::*}, schema::store_hours::{self, dsl::*}, schema::store_address::{self, dsl::*}};
use chrono::{NaiveTime};
use crate::diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StoreId {
    pub store_id: i32
}

#[derive(Deserialize)]
pub struct StoreHourId {
    pub store_hour_id: i32
}

#[derive(Deserialize)]
pub struct StoreAddressId {
    pub store_address_id: i32
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "store"]
pub struct StoreCreate {
    pub name: String
}

#[derive(Eq, PartialEq, Identifiable, Serialize, Deserialize, Queryable)]
#[primary_key(store_id)]
#[table_name = "store"]
pub struct Store {
    pub store_id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct FullStore {
    pub name: String,
    pub address: StoreAddress,
    pub hours: Vec<StoreHours>
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "store_address"]
pub struct StoreAddressCreate {
    pub store_id: i32,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub zip: i32,
    pub phone: String,
    pub email: String
}

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, AsChangeset, Debug)]
#[belongs_to(Store)]
#[primary_key(store_address_id)]
#[table_name = "store_address"]
pub struct StoreAddress {
    pub store_address_id: i32,
    pub store_id: i32,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub zip: i32,
    pub phone: String,
    pub email: String
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "store_hours"]
pub struct StoreHoursCreate {
    pub store_id: i32,
    pub day_of_week: i32,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>
}

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, AsChangeset, Debug, Copy, Clone)]
#[belongs_to(Store)]
#[primary_key(store_hours_id)]
#[table_name = "store_hours"]
pub struct StoreHours {
    pub store_hours_id: i32,
    pub store_id: i32,
    pub day_of_week: i32,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct StoreWithHours {
    pub store: Store,
    pub store_hours: Vec<StoreHours>
}

impl Store {
    pub fn find_all() -> QueryResult<Vec<Self>> {
        let conn = db::establish_connection();
        store.order(store::store_id.asc()).load::<Self>(&conn)
    }

    pub fn find_all_data(id: i32) -> QueryResult<FullStore> {
        let conn = db::establish_connection();

        let store_name = store
            .filter(store::store_id
            .eq(id))
            .first::<Self>(&conn)?;
        let store_address_data = store_address
            .filter(store_address::store_id
            .eq(id))
            .first::<StoreAddress>(&conn)?;
        let store_hours_data = store_hours
            .filter(store_hours::store_id
            .eq(id))
            .load::<StoreHours>(&conn)?;

        let full_store_data = FullStore {
            name: store_name.name,
            address: store_address_data,
            hours: store_hours_data
        };

        Ok(full_store_data)
    }

    pub fn find(id: i32) -> QueryResult<Self> {
        let conn = db::establish_connection();
    
        store.filter(store::store_id.eq(id)).first::<Self>(&conn)
    }

    pub fn find_all_store_hours() -> QueryResult<Vec<StoreWithHours>> {
        let conn = db::establish_connection();

        let store_list = store::table.load::<Store>(&conn)?;
        let store_hours_list = StoreHours::belonging_to(&store_list)
            .load::<StoreHours>(&conn)?
            .grouped_by(&store_list);
        
        let mut store_final_list: Vec<StoreWithHours> = vec![];

        let mut temp_iter = 0;
        for store_info in store_list {
            let current_store = StoreWithHours {
                store: store_info,
                store_hours: store_hours_list[temp_iter].clone()
            };

            store_final_list.push(current_store);
            temp_iter += 1;
        }

        Ok(store_final_list)
    }

    pub fn find_store_hours(id: i32) -> QueryResult<StoreWithHours> {
        let conn = db::establish_connection();

        let store_member = store
            .filter(store::store_id
            .eq(id))
            .first::<Self>(&conn)?;
        let store_member_hours = store_hours
            .filter(store_hours::store_id.eq(id))
            .load::<StoreHours>(&conn)?;

        let store_hour = StoreWithHours {
            store: store_member,
            store_hours: store_member_hours,
        };

        Ok(store_hour)
    }

    pub fn find_address(id: i32) -> QueryResult<StoreAddress> {
        let conn = db::establish_connection();

        let store_address_details = store_address.filter(store_address::store_id.eq(id)).first::<StoreAddress>(&conn)?;

        Ok(store_address_details)
    }

    pub fn create(store_create: StoreCreate) -> QueryResult<Self> {
        let conn = db::establish_connection();

        let store_created: Self = diesel::insert_into(store::table)
            .values(store_create)
            .get_result(&conn)?;

        // generate store hours
        let mut hours_list: Vec<StoreHoursCreate> = vec![];

        for x in 0..7 {
            let day_hours = StoreHoursCreate {
                store_id: store_created.store_id,
                day_of_week: x,
                start_time: None,
                end_time: None,
            };

            hours_list.push(day_hours);
        }

        diesel::insert_into(store_hours::table)
            .values(hours_list)
            .execute(&conn)?;

        Ok(store_created)
    }

    pub fn create_address(store_address_create: StoreAddressCreate) -> QueryResult<StoreAddress> {
        let conn = db::establish_connection();

        let store_address_created: StoreAddress = diesel::insert_into(store_address::table)
            .values(store_address_create)
            .get_result(&conn)?;
        
        Ok(store_address_created)
    }

    pub fn update(id: i32, store_update: StoreCreate) -> Result<Self, ApiError> {
        let conn = db::establish_connection();

        let store_updated = diesel::update(store::table)
            .filter(store::store_id.eq(id))
            .set(store_update)
            .get_result(&conn)?;

        Ok(store_updated)
    }

    pub fn update_address(id: i32, store_address_update: StoreAddressCreate) -> Result<StoreAddress, ApiError> {
        let conn = db::establish_connection();

        let store_updated = diesel::update(store_address::table)
            .filter(store_address::store_id.eq(id))
            .set(store_address_update)
            .get_result(&conn)?;

        Ok(store_updated)
    }

    pub fn update_one_hour(id: i32, store_hour: StoreHoursCreate) -> Result<StoreHours, ApiError> {
        let conn = db::establish_connection();

        let store_hour_updated = diesel::update(store_hours::table)
            .filter(store_hours::store_hours_id.eq(id))
            .set(store_hour)
            .get_result(&conn)?;

        Ok(store_hour_updated)
    }

    pub fn update_hours(store_hours_update: Vec<StoreHoursCreate>) -> Result<(), ApiError> {
        let conn = db::establish_connection();

        for store_member in store_hours_update {
            diesel::update(store_hours::table)
                .filter(store_hours::store_id.eq(store_member.store_id))
                .filter(store_hours::day_of_week.eq(store_member.day_of_week))
                .set(store_member).execute(&conn);
        }

        Ok(())
    }

    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let conn = db::establish_connection();

        // also make sure to delete other store data
        diesel::delete(store_hours::table)
            .filter(store_hours::store_id
            .eq(id))
            .execute(&conn)?;
        diesel::delete(store_address::table)
            .filter(store_address::store_id
            .eq(id))
            .execute(&conn)?;

        let res = diesel::delete(
                store::table
                    .filter(store::store_id
                    .eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}