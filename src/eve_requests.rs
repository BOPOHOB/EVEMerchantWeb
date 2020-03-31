fn get_env_variable(key: &str)->String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("{} {}", key, e),
    }
}

pub fn login(authorization_code: &String) -> Result<json::JsonValue, json::JsonValue> {
    let authentication_url = "https://login.eveonline.com/oauth/token";
    let combo =  base64::encode(String::from(format!("{}:{}", get_env_variable("EVE_CLIENT_ID"), get_env_variable("EVE_CLIENT_SECRET"))));

    let request = reqwest::Client::new().post(authentication_url)
        .header("Authorization", format!("Basic {}", combo))
        .header("User-Agent", "bopohob merchant monitor")
        .header("Content-Type", "application/json")
        .body(json::object!{
            grant_type: "authorization_code",
            code: format!("{}", authorization_code)
        }.dump())
        .send();

    let mut runtime = tokio::runtime::Runtime::new().expect("Login request get tokio runtime");
    let responce = runtime.block_on(request).expect("Login attempt");
    let is_success = responce.status().is_success();
    let responce_text = runtime.block_on(responce.text()).expect("Login attempt responce body");
    let result = json::parse(responce_text.as_str()).expect("Login responce json parse");
    if is_success {
        Ok(result)
    } else {
        Err(result)
    }
}

pub fn verify(access_token: & String) -> Result<json::JsonValue, json::JsonValue> {
    let verify_url = "https://login.eveonline.com/oauth/verify";

    let request = reqwest::Client::new().get(verify_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "bopohob merchant monitor")
        .header("Content-Type", "application/json")
        .send();

    let mut runtime = tokio::runtime::Runtime::new().expect("Login request get tokio runtime");
    let responce = runtime.block_on(request).expect("Login attempt");
    let is_success = responce.status().is_success();
    let responce_text = runtime.block_on(responce.text()).expect("Login attempt responce body");
    let result = json::parse(responce_text.as_str()).expect("Login responce json parse");
    if is_success {
        Ok(result)
    } else {
        Err(result)
    }
}
