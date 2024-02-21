#[cfg(test)]
pub mod test {
    use std::env;

    use crate::get_pool;
    use crate::routes;
    use actix_http::Request;
    use actix_service::Service;
    use actix_web::{
        dev::ServiceResponse,
        test::{self},
        web, App, Error,
    };
    use dotenv;

    pub async fn init() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        dotenv::from_filename(".env").expect(".env file doesnt exists");
        dotenv::dotenv().ok();

        env::var("DATABASE_URL").expect("DATABASE_URL not set");
        env::var("SECRET_JWT").expect("SECRET_JWT not set");

        let pool = get_pool().await;

        test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes::user_routes)
                .configure(routes::auth_routes),
        )
        .await
    }
}
