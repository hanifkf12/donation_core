use crate::application::services::user_service::UserService;
use crate::application::use_cases::contracts::Command;
use crate::application::use_cases::users::get_users::GetUsersUseCase;
use crate::application::use_cases::users::create_user::{CreateUserRequest, CreateUserUseCase};
use crate::infrastructure::database::repositories::sqlx_product_repository::SqlxProductRepositoryImpl;
use actix_web::{HttpResponse, Responder, web};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn get_users_route(pool: web::Data<PgPool>) -> impl Responder {
    let user_repo = Arc::new(SqlxProductRepositoryImpl);
    let user_service = UserService::new(user_repo);

    match GetUsersUseCase.execute(&user_service, pool.get_ref(), ()).await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: users,
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

pub async fn create_user_route(pool: web::Data<PgPool>, request: web::Json<CreateUserRequest>) -> impl Responder {
    let user_repo = Arc::new(SqlxProductRepositoryImpl);
    let user_service = UserService::new(user_repo);

    match CreateUserUseCase.execute(&user_service, pool.get_ref(), request.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(ApiResponse {
            success: true,
            data: Some(user),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}
