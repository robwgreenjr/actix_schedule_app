#[derive(Queryable)]
pub struct Store {
    pub store_id: i32,
    pub name: String,
    pub address_id: i32
}

#[derive(Queryable)]
pub struct StoreAddress {
    pub store: Store,
    pub street_address: String,
    pub city: String,
    pub zip: i32,
    pub phone: String,
    pub email: String
}