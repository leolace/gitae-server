use crate::auth::{auth_controller, auth_dto};
use crate::error::Error;
use crate::models::{auth::AuthClaims, auth::AuthPayload, user::User};
use crate::user::user_service::UserService;
use crate::{AppPool, ResultE};
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use chrono::Local;
use hmac::{Hmac, Mac};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sha2::Sha256;
use sqlx::Row;
use std::time::Duration;

pub struct AuthService {
    pool: AppPool,
}

// TODO: change jwt lib to a better and more recent one (jsonwebtoken)
impl AuthService {
    pub fn new(pool: AppPool) -> AuthService {
        AuthService { pool }
    }

    pub async fn sign_up(&self, body: web::Json<auth_dto::SignUp>) -> ResultE<User> {
        if body.email.is_empty() || body.username.is_empty() || body.password.is_empty() {
            return Err(Error::new(
                StatusCode::BAD_REQUEST,
                "All fields must be set",
            ));
        }

        let user_service = UserService::new(self.pool.clone()).await;

        let user_exists_by_email = user_service.exists_by_email(&body.email).await;

        if user_exists_by_email {
            return Err(Error::new(
                StatusCode::CONFLICT,
                "This email has already been taken",
            ));
        }

        let user_exists_by_username = user_service.exists_by_username(&body.username).await;

        if user_exists_by_username {
            return Err(Error::new(
                StatusCode::CONFLICT,
                "This username has already been taken",
            ));
        }

        let pool = self.pool.get_ref();
        let query = sqlx::query(
            "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&body.username)
        .bind(&body.email)
        .bind(&body.password)
        .fetch_one(pool)
        .await
        .unwrap();

        let user = User::from_row(query);

        Ok(user)
    }

    pub async fn sign_in(&self, body: web::Json<auth_dto::SignIn>) -> ResultE<AuthPayload> {
        let user_exists = UserService::new(self.pool.clone())
            .await
            .find_by_email(&body.email)
            .await;

        match user_exists {
            Some(d) => {
                if d.password != body.password {
                    return Err(Error::new(StatusCode::BAD_REQUEST, "Credentials are wrong"));
                }

                let now = Local::now();
                let exp = (now + Duration::from_secs(10800)).timestamp();

                let user_claims = AuthClaims { user_id: d.id, exp };

                let token = encode(
                    &Header::default(),
                    &user_claims,
                    &EncodingKey::from_secret("secret".as_ref()),
                )
                .unwrap();

                Ok(AuthPayload { token })
            }
            None => Err(Error::new(StatusCode::BAD_REQUEST, "Credentials are wrong")),
        }
    }

    pub async fn me(&self, req: HttpRequest) -> ResultE<User> {
        let req_headers = req.headers();
        let auth_header = req_headers.get("Authorization").unwrap();
        let auth = auth_header
            .to_str()
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>()[1];

        let user_payload: AuthClaims = match decode::<AuthClaims>(
            &auth,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(d) => d.claims,
            Err(_) => return Err(Error::new(StatusCode::BAD_REQUEST, "Invalid bearer token")),
        };

        let user = UserService::new(self.pool.clone())
            .await
            .find(user_payload.user_id)
            .await;

        match user {
            Some(user) => Ok(user),
            None => Err(Error::new(StatusCode::NOT_FOUND, "Bad request user")),
        }
    }
}
