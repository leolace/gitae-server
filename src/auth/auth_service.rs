use crate::auth::{auth_controller, auth_dto};
use crate::error::Error;
use crate::models::{auth::JwtPayload, auth::UserId, user::User};
use crate::user::user_service::UserService;
use crate::AppPool;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use sqlx::Row;

type ResultE<T, E = Error> = Result<T, E>;

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

    pub async fn sign_in(&self, body: web::Json<auth_dto::SignIn>) -> ResultE<JwtPayload> {
        let user_exists = UserService::new(self.pool.clone())
            .await
            .find_by_email(&body.email)
            .await;

        match user_exists {
            Some(d) => {
                if d.password != body.password {
                    return Err(Error::new(StatusCode::BAD_REQUEST, "Credentials are wrong"));
                }
                let user = UserId { id: d.id };
                let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret key").unwrap();
                let token = user.sign_with_key(&key).unwrap();

                Ok(JwtPayload { token })
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
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret key").unwrap();

        let user_payload: UserId = match auth.verify_with_key(&key) {
            Ok(d) => d,
            Err(_) => return Err(Error::new(StatusCode::BAD_REQUEST, "Invalid bearer token")),
        };

        let user = UserService::new(self.pool.clone())
            .await
            .find(user_payload.id)
            .await;

        match user {
            Some(user) => Ok(user),
            None => Err(Error::new(StatusCode::NOT_FOUND, "Bad request user")),
        }
    }
}
