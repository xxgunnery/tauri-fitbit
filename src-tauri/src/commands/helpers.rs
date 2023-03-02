use hyper::header::{AUTHORIZATION, CONNECTION, CONTENT_TYPE};
use reqwest::{Error, Response};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{Read, Write},
    mem::drop,
};

use super::get_fitbit_auth;

pub fn get_encoded_pkce(pkce: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pkce.as_bytes());
    let hash = hasher.finalize();
    let encoded = base64_url::encode(&hash[..]);
    return encoded;
}

pub fn get_fitbit_auth_url(encoded_value: String, window: tauri::Window) {
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_state = env::var("CLIENT_STATE").unwrap();

    let url = format!(
        "https://www.fitbit.com/oauth2/authorize?client_id={}&response_type=code&code_challenge={}&code_challenge_method=S256&scope=cardio_fitness%20heartrate+location%20oxygen_saturation%20respiratory_rate%20sleep+weight&state={}",
        client_id, encoded_value, client_state
    );

    window.eval(&format!("window.location.replace('{}')", url));
}

pub async fn refresh_access_token(refresh_token: String) -> (String, String) {
    println!("REFRESHING ACCESS TOKEN");
    let base_url = env::var("API_BASE_URL").unwrap();
    let url = base_url + "/oauth2/token";

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();

    let client_id_secret = &format!("{}:{}", client_id, client_secret);
    let authorization_header = "Basic ".to_string() + &base64_url::encode(client_id_secret);
    println!("{:?}{:?}", authorization_header, refresh_token);

    let grant_type = &"refresh_token".to_string();

    let mut params = HashMap::new();

    let param_data: [(&str, &String); 3] = [
        ("grant_type", grant_type),
        ("client_id", &client_id),
        ("refresh_token", &refresh_token),
    ];
    params.extend(param_data.iter().cloned());

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(AUTHORIZATION, authorization_header)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await;

    let json_string = response.unwrap().text().await.unwrap();
    println!("{:?}", json_string);
    let json_data: Value = serde_json::from_str(&json_string).unwrap();

    let json_object = json_data.as_object().unwrap();

    let refresh_token = json_object.get("refresh_token").unwrap().to_string();
    let access_token = json_object.get("access_token").unwrap().to_string();

    (refresh_token, access_token)
}

pub async fn make_fitbit_api_call(url: String, window: tauri::Window) -> Result<Response, Error> {
    {
        println!("GETTING FROM API");
        let mut file = File::open("../public/data/user_data.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let json_data: Value = serde_json::from_str(&contents).unwrap();
        let json_object = json_data.as_object().unwrap();
        let access_token = json_object.get("access_token").unwrap().as_str().unwrap();

        let authorization_header = "Bearer ".to_string() + access_token;

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(AUTHORIZATION, authorization_header)
            .header(CONTENT_TYPE, "application/json;charset=UTF-8")
            .header(CONNECTION, "keep-alive")
            .send()
            .await;

        let status = response.as_ref().unwrap().status();
        if status == 401 {
            println!("AUTHORIZING FITBIT");
            get_fitbit_auth(window);
            // let refresh_token = json_object.get("refresh_token").unwrap().to_string();
            // let (new_refresh_token, new_access_token) = refresh_access_token(refresh_token).await;
            // file.write_all(
            //     format!(
            //         "{{\"refresh_token\": \"{}\", \"access_token\": \"{}\"}}",
            //         new_refresh_token, new_access_token
            //     )
            //     .as_bytes(),
            // )
            // .unwrap();
            // return response;
        }
        return response;
    }
}
