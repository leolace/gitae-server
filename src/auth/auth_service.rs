use crate::auth::{auth_controller, auth_dto};
use crate::error::HttpError;
use crate::models::{auth::AuthClaims, auth::AuthPayload, user::User};
use crate::user::user_service::{self, UserService};
use crate::{AppPool, ResultE};
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use chrono::Local;
use hmac::{Hmac, Mac};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sqlx::Row;
use std::time::Duration;
use uuid::Uuid;

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
            return Err(HttpError::new(
                StatusCode::BAD_REQUEST,
                "All fields must be set",
            ));
        }

        let user_service = UserService::new(self.pool.clone()).await;

        let user_exists_by_email = user_service.exists_by_email(&body.email).await;

        if user_exists_by_email {
            return Err(HttpError::new(
                StatusCode::CONFLICT,
                "This email has already been taken",
            ));
        }

        let user_exists_by_username = user_service.exists_by_username(&body.username).await;

        if user_exists_by_username {
            return Err(HttpError::new(
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
            Some(user) => {
                if user.password != body.password {
                    return Err(HttpError::new(
                        StatusCode::BAD_REQUEST,
                        "Credentials are wrong",
                    ));
                }

                let now = Local::now();
                let exp = (now + Duration::from_secs(10800)).timestamp();

                let user_claims = AuthClaims {
                    user_id: user.id,
                    exp,
                };

                let token = encode(
                    &Header::default(),
                    &user_claims,
                    &EncodingKey::from_secret("secret".as_ref()),
                )
                .unwrap();

                match self.open_session(&token, &user.id).await {
                    Ok(_) => Ok(AuthPayload { token }),
                    Err(e) => Err(HttpError::new(StatusCode::BAD_REQUEST, e)),
                }
            }
            None => Err(HttpError::new(
                StatusCode::BAD_REQUEST,
                "Credentials are wrong",
            )),
        }
    }

    pub async fn me(&self, req: HttpRequest) -> ResultE<User> {
        let req_headers = req.headers();
        let auth_header = match req_headers.get("authorization") {
            Some(header) => header,
            None => {
                return Err(HttpError::new(
                    StatusCode::BAD_REQUEST,
                    "Authorization header not found",
                ))
            }
        };

        let token = match auth_header.to_str().unwrap().split(' ').last() {
            Some(v) => v,
            None => {
                return Err(HttpError::new(
                    StatusCode::BAD_REQUEST,
                    "Invalid bearer token format",
                ))
            }
        };

        let session = sqlx::query("SELECT * FROM sessions WHERE token = ($1)")
            .bind(token)
            .fetch_one(self.pool.get_ref())
            .await;

        let session = match session {
            Ok(s) => s,
            Err(_) => return Err(HttpError::new(StatusCode::UNAUTHORIZED, "Usuário não autenticado"))
        };

        let token_from_session = session.get::<String, &str>("token");

        match token_from_session == token {
            true => (),
            false => return Err(HttpError::new(StatusCode::UNAUTHORIZED, "Token inválido"))
        }

        println!("token: {}; session: {}", token, token_from_session);

        let user_payload: AuthClaims = match decode::<AuthClaims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(d) => d.claims,
            Err(_) => {
                return Err(HttpError::new(
                    StatusCode::BAD_REQUEST,
                    "Invalid bearer token",
                ))
            }
        };


        let user = UserService::new(self.pool.clone())
            .await
            .find(user_payload.user_id)
            .await;

        match user {
            Some(user) => Ok(user),
            None => Err(HttpError::new(StatusCode::NOT_FOUND, "Bad request user")),
        }
    }

    async fn get_session_by_user_id(&self, user_id: &Uuid) -> Result<Uuid, String> {
        let pool = self.pool.get_ref();

        let query = sqlx::query("SELECT id FROM sessions WHERE user_id = ($1)")
            .bind(user_id)
            .fetch_one(pool)
            .await;

        match query {
            Ok(query_result) => Ok(query_result.get::<Uuid, &str>("id")),
            Err(_) => Err(String::from("Session doesnt exists")),
        }
    }

    async fn open_session(&self, token: &String, user_id: &Uuid) -> Result<(), &str> {
        let pool = self.pool.get_ref();
        let session = self.get_session_by_user_id(user_id).await;

        println!("{:?}", session);

        if session.is_ok() {
            println!("deletar");
            self.delete_session(session.unwrap()).await.unwrap();
        }

        let query = sqlx::query("INSERT INTO sessions (token, user_id) VALUES ($1, $2)")
            .bind(token)
            .bind(user_id)
            .execute(pool)
            .await;

        match query {
            Ok(_) => Ok(()),
            Err(_) => Err("Error opening session"),
        }
    }

    async fn delete_session(&self, session_id: Uuid) -> Result<(), &str> {
        let pool = self.pool.get_ref();

        let query = sqlx::query("DELETE FROM sessions WHERE id = ($1)")
            .bind(session_id)
            .execute(pool)
            .await;

        println!("{:?}", query);

        match query {
            Ok(_) => Ok(()),
            Err(_) => Err("Was not possivel delete session"),
        }
    }
}
