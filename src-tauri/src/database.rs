mod connect;
mod db_model;
pub mod schema;

use crate::commands::{HeartFeedDay, SleepFeedDay};
pub use connect::{establish_connection, PostgresPool};
pub use db_model::*;
use diesel::prelude::*;
use schema::heartdata;
use schema::heartdata::dsl::*;
use schema::sleepdata;
use schema::sleepdata::dsl::*;

pub fn get_all_heart_data(connection: &mut PgConnection) -> Vec<HeartRateData> {
    let heart_rate_data = heartdata
        .load::<HeartRateData>(connection)
        .expect("Error loading heart rate data");

    return heart_rate_data;
}

pub fn store_heart_data(connection: &mut PgConnection, heart_data: &Vec<HeartFeedDay>) {
    let heart_data: Vec<NewHeartRateData> = heart_data
        .iter()
        .map(|val| {
            let _date = &val.date;
            let heart_rate = val.heart_rate;
            NewHeartRateData {
                date_time: _date.to_string(),
                resting_rate: heart_rate,
            }
        })
        .collect();

    diesel::insert_into(heartdata::table)
        .values(heart_data)
        .on_conflict(date_time)
        .do_nothing()
        .execute(connection)
        .expect("Error saving heart data.");
}

pub fn get_most_recent_heart_data(connection: &mut PgConnection) -> String {
    let response = heartdata
        .select(date_time)
        .order(date_time.desc())
        .limit(1)
        .load::<String>(connection)
        .expect("Error loading heart rate data");

    if !response.is_empty() {
        let most_recent_value = response[0].clone();
        return most_recent_value
    } else {
        return "".to_string();
    }
}

pub fn get_all_sleep_data(connection: &mut PgConnection) -> Vec<SleepDataFromDB> {
    let sleep_data = sleepdata
        .load::<SleepDataFromDB>(connection)
        .expect("Error loading sleep data");

    return sleep_data;
}

pub fn store_sleep_data(connection: &mut PgConnection, sleep_data: &Vec<SleepFeedDay>) {
    let sleep_data: Vec<NewSleepData> = sleep_data
        .iter()
        .map(|val| NewSleepData {
            sleep_date: val.sleep_date.to_string(),
            efficiency: val.efficiency,
            end_time: val.end_time.to_string(),
            deep: val.deep,
            light: val.light,
            rem: val.rem,
            wake: val.wake,
        })
        .collect();

    diesel::insert_into(sleepdata::table)
        .values(sleep_data)
        .on_conflict(sleep_date)
        .do_nothing()
        .execute(connection)
        .expect("Error saving heart data.");
}
