table! {
    staff (staff_id) {
        staff_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        password -> Text,
        email -> Text,
        phone -> Nullable<Text>,
        access -> Nullable<Text>,
        calendar_color -> Nullable<Text>,
    }
}

table! {
    staff_hours (staff_hours_id) {
        staff_hours_id -> Int4,
        staff_id -> Int4,
        day_of_week -> Int4,
        start_time -> Nullable<Time>,
        end_time -> Nullable<Time>,
    }
}

table! {
    store (store_id) {
        store_id -> Int4,
        name -> Varchar,
        store_address_id -> Int4,
    }
}

table! {
    store_address (store_address_id) {
        store_address_id -> Int4,
        store_id -> Int4,
        street_address -> Varchar,
        city -> Varchar,
        state -> Varchar,
        zip -> Int4,
        phone -> Varchar,
        email -> Varchar,
    }
}

table! {
    store_hours (store_hours_id) {
        store_hours_id -> Int4,
        store_id -> Int4,
        day_of_week -> Int4,
        start_time -> Nullable<Time>,
        end_time -> Nullable<Time>,
    }
}

joinable!(staff_hours -> staff (staff_id));

allow_tables_to_appear_in_same_query!(
    staff,
    staff_hours,
    store,
    store_address,
    store_hours,
);
