pub mod api_model;
pub mod helpers;
use crate::database::{self, store_sleep_data};
use chrono::{DateTime, Local, Months};

pub use api_model::*;
pub use database::{
    establish_connection, get_all_heart_data, get_most_recent_heart_data, store_heart_data,
    HeartRateData, PostgresPool,
};
pub use helpers::*;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

use tauri::State;

#[tauri::command]
pub fn get_fitbit_auth(window: tauri::Window) {
    println!("GETTING FITBIT AUTH DATA \n");

    let pkce: String = env::var("CODE_VERIFIER").unwrap();
    let encoded_pkce: String = get_encoded_pkce(pkce);

    return get_fitbit_auth_url(encoded_pkce, window);
}

#[tauri::command]
pub async fn get_fitbit_token(fitbit_code: String) {
    println!("GETTING FITBIT TOKEN");
    let code = &fitbit_code;

    let base_url = "https://api.fitbit.com/oauth2/token";

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let code_verifier = &env::var("CODE_VERIFIER").unwrap();

    let client_id_secret = &format!("{}:{}", client_id, client_secret);
    let authorization_header = "Basic ".to_string() + &base64_url::encode(client_id_secret);

    let grant_type = &"authorization_code".to_string();

    let mut params = HashMap::new();

    let param_data: [(&str, &String); 4] = [
        ("client_id", &client_id),
        ("grant_type", grant_type),
        ("code", code),
        ("code_verifier", code_verifier),
    ];
    params.extend(param_data.iter().cloned());

    let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();
    let response = client
        .post(base_url)
        .header(AUTHORIZATION, authorization_header)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await;

    let json_string = response.unwrap().text().await.unwrap();
    let json_data: Value = serde_json::from_str(&json_string).unwrap();

    let json_object = json_data.as_object().unwrap();

    println!("JSON OBJECT W/ TOKEN: {:?} \n", json_object);

    let refresh_token = json_object.get("refresh_token").unwrap().as_str().unwrap();
    let access_token = json_object.get("access_token").unwrap().as_str().unwrap();

    let mut file = File::create("../public/data/user_data.json").unwrap();
    file.write_all(
        format!(
            "{{\"refresh_token\": \"{}\", \"access_token\": \"{}\"}}",
            refresh_token, access_token
        )
        .as_bytes(),
    )
    .unwrap();
}

#[tauri::command(async)]
pub async fn get_heart_data(
    state: State<'_, PostgresPool>,
    window: tauri::Window,
) -> Result<HeartFeed, ()> {
    let connection = &mut state.get().unwrap();

    let most_recent_date = get_most_recent_heart_data(connection);

    let now: DateTime<Local> = Local::now();
    let date_string = now.format("%Y-%m-%d").to_string();

    if most_recent_date == date_string {
        let all_data = get_all_heart_data(connection);

        let data = all_data
            .iter()
            .map(|val| {
                let date = &val.date_time;
                let heart_rate = val.resting_rate;
                HeartFeedDay {
                    date: date.to_string(),
                    heart_rate: heart_rate,
                }
            })
            .collect();

        println!("Returning from DB");
        return Ok(data);
    }

    let base_url = env::var("API_BASE_URL").unwrap();
    let url = format!(
        "{}{}",
        base_url, "1/user/-/activities/heart/date/2023-01-01/today.json"
    );

    let response = make_fitbit_api_call(url, window).await;
    let json_string = response.unwrap().text().await.unwrap();
    let json_data: HeartRateResponse = serde_json::from_str(&json_string).unwrap();
    let all_heart_activity = json_data.activities_heart;
    let mut resting_heartrates = [].to_vec();
    for activity in all_heart_activity {
        let get_value = activity.value.unwrap();
        let date = activity.date_time;
        if get_value.resting_heartrate.is_some() && date.is_some() {
            let resting_rate = get_value.resting_heartrate;
            let resting_data = HeartFeedDay {
                heart_rate: resting_rate.unwrap(),
                date: date.unwrap(),
            };
            resting_heartrates.push(resting_data);
        }
    }
    store_heart_data(connection, &resting_heartrates);
    let returned_data = HeartFeed {
        data: resting_heartrates,
    };
    return Ok(returned_data);
}

#[tauri::command(async)]
pub async fn get_sleep_data(
    state: State<'_, PostgresPool>,
    window: tauri::Window,
) -> Result<SleepFeed, ()> {
    let connection = &mut state.get().unwrap();

    //let most_recent_date = get_most_recent_heart_data(connection);

    let now: DateTime<Local> = Local::now();
    let date_string = now.format("%Y-%m-%d").to_string();
    let start_date = now - Months::new(2);
    let start_date_string = start_date.format("%Y-%m-%d").to_string();
    // if most_recent_date == date_string {
    //     let all_data = get_all_sleep_data();

    //     let data = all_data
    //         .iter()
    //         .map(|val| {
    //             let date = &val.date_time;
    //             let heart_rate = val.resting_rate;
    //             HeartFeedDay {
    //                 date: date.to_string(),
    //                 heart_rate: heart_rate,
    //             }
    //         })
    //         .collect();

    //     println!("Returning from DB");
    //     return data;
    // }

    let base_url = env::var("API_BASE_URL").unwrap();
    let url =
        base_url + "1.2/user/-/sleep/date/" + &start_date_string + "/" + &date_string + ".json";

    let response = make_fitbit_api_call(url, window).await;

    let json_string = response.unwrap().text().await.unwrap();
    let json_data: SleepResponse = serde_json::from_str(&json_string).unwrap();
    let all_sleep_data = json_data.sleep;
    let mut sleep_data = [].to_vec();
    for sleep_day in all_sleep_data {
        let efficiency = sleep_day.efficiency;
        let date = sleep_day.date;
        let end_time = sleep_day.end_time;
        let summ = sleep_day.levels.summary;
        if summ.rem.is_some() && summ.deep.is_some() && summ.light.is_some() && summ.wake.is_some()
        {
            let sleep = SleepFeedDay {
                sleep_date: date.unwrap(),
                end_time: end_time.unwrap(),
                efficiency: efficiency.unwrap(),
                rem: summ.rem.unwrap().minutes,
                wake: summ.wake.unwrap().minutes,
                deep: summ.deep.unwrap().minutes,
                light: summ.light.unwrap().minutes,
            };
            sleep_data.push(sleep)
        }
    }
    sleep_data.reverse();
    store_sleep_data(connection, &sleep_data);
    return Ok(SleepFeed { data: sleep_data });
}
