// @generated automatically by Diesel CLI.

diesel::table! {
    heartdata (id) {
        id -> Int4,
        date_time -> Text,
        resting_rate -> Int4,
    }
}

diesel::table! {
    sleepdata (id) {
        id -> Int4,
        sleep_date -> Text,
        efficiency -> Int4,
        end_time -> Text,
        rem -> Int4,
        light -> Int4,
        deep -> Int4,
        wake -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    heartdata,
    sleepdata,
);
