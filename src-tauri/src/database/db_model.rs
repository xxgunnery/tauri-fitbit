use crate::database::schema::{heartdata, sleepdata};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct HeartRateData {
    pub id: i32,
    pub date_time: String,
    pub resting_rate: i32,
}

#[derive(Insertable)]
#[diesel(table_name = heartdata)]
pub struct NewHeartRateData {
    pub date_time: String,
    pub resting_rate: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct SleepDataFromDB {
    pub id: i32,
    pub sleep_date: String,
    pub efficiency: i32,
    pub end_time: String,
    pub rem: i32,
    pub light: i32,
    pub deep: i32,
    pub wake: i32,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = sleepdata)]
pub struct NewSleepData {
    pub sleep_date: String,
    pub efficiency: i32,
    pub end_time: String,
    pub rem: i32,
    pub light: i32,
    pub deep: i32,
    pub wake: i32,
}
