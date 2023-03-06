use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct HeartRateResponse {
    #[serde(rename = "activities-heart")]
    pub activities_heart: Vec<HeartData>,
}

#[derive(Deserialize, Debug)]
pub struct HeartData {
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,
    pub value: Option<HeartValue>,
}

#[derive(Deserialize, Debug)]
pub struct HeartValue {
    #[serde(rename = "customHeartRateZone")]
    pub custom_heartrate_zone: Option<HeartZone>,
    #[serde(rename = "HeartRateZone")]
    pub heartrate_zone: Option<HeartZone>,
    #[serde(rename = "restingHeartRate")]
    pub resting_heartrate: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct HeartZone {
    #[serde(rename = "caloriesOut")]
    pub calories_out: i32,
    pub max: i32,
    pub min: i32,
    pub minutes: i32,
    pub name: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct HeartFeed {
    pub data: Vec<HeartFeedDay>,
}
#[derive(Debug, Clone, Serialize)]
pub struct HeartFeedDay {
    pub heart_rate: i32,
    pub date: String,
}

impl FromIterator<HeartFeedDay> for HeartFeed {
    fn from_iter<I: IntoIterator<Item = HeartFeedDay>>(iter: I) -> Self {
        let days = iter.into_iter().collect();
        HeartFeed { data: days }
    }
}

#[derive(Deserialize, Debug)]
pub struct SleepResponse {
    pub sleep: Vec<SleepData>,
}

#[derive(Deserialize, Debug)]
pub struct SleepData {
    #[serde(rename = "dateOfSleep")]
    pub date: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub efficiency: Option<i32>,
    pub levels: SleepLevels,
}

#[derive(Deserialize, Debug)]
pub struct SleepLevels {
    pub summary: Levels,
}

#[derive(Deserialize, Debug)]
pub struct Levels {
    pub deep: Option<LevelsData>,
    pub light: Option<LevelsData>,
    pub rem: Option<LevelsData>,
    pub wake: Option<LevelsData>,
}

#[derive(Deserialize, Debug)]
pub struct LevelsData {
    pub count: i32,
    pub minutes: i32,
}


#[derive(Debug, Clone, Serialize)]
pub struct SleepFeed {
    pub data: Vec<SleepFeedDay>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SleepFeedDay {
    pub sleep_date: String,
    pub end_time: String,
    pub efficiency: i32,
    pub deep: i32,
    pub light: i32,
    pub rem: i32,
    pub wake: i32,
}

impl FromIterator<SleepFeedDay> for SleepFeed {
    fn from_iter<I: IntoIterator<Item = SleepFeedDay>>(iter: I) -> Self {
        let days = iter.into_iter().collect();
        SleepFeed { data: days }
    }
}