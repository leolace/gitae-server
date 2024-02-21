use actix_web::http::header::HeaderMap;


pub fn get_token(headers: &HeaderMap) -> Result<String, &str> {
    let auth_header = match headers.get("authorization") {
        Some(header) => header,
        None => return Err("Authorization header not found"),
    };

    let token = match auth_header.to_str().unwrap().split(' ').last() {
        Some(token) => token,
        None => return Err("Invalid bearer token format"),
    };

    Ok(String::from(token))
}
