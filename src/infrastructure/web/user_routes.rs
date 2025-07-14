use crate::application::services::user_service::UserService;
use crate::application::use_cases::contracts::Command;
use crate::application::use_cases::users::create_user::{CreateUserRequest, CreateUserUseCase};
use crate::application::use_cases::users::delete_user::DeleteUserUseCase;
use crate::application::use_cases::users::get_users::{GetUserByIdUseCase, GetUsersUseCase};
use crate::application::use_cases::users::update_user::UpdateUserUseCase;
use crate::infrastructure::database::repositories::sqlx_user_repository::SqlxUserRepositoryImpl;
use actix_web::{HttpResponse, Responder, web};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

pub async fn get_users_route(pool: web::Data<PgPool>) -> impl Responder {
    let user_repo = Arc::new(SqlxUserRepositoryImpl);
    let user_service = UserService::new(user_repo);

    match GetUsersUseCase
        .execute(&user_service, pool.get_ref(), ())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(ApiResponse {
            message: Some("Users fetched successfully".to_string()),
            success: true,
            data: users,
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            message: Some("Failed to fetch users".to_string()),
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

pub async fn create_user_route(
    pool: web::Data<PgPool>,
    request: web::Json<CreateUserRequest>,
) -> impl Responder {
    let user_repo = Arc::new(SqlxUserRepositoryImpl);
    let user_service = UserService::new(user_repo);

    match CreateUserUseCase
        .execute(&user_service, pool.get_ref(), request.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Created().json(ApiResponse {
            message: Some("User created successfully".to_string()),
            success: true,
            data: Some(user),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            message: Some("Failed to create user".to_string()),
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

pub async fn get_user_by_id_route(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    let user_repo = Arc::new(SqlxUserRepositoryImpl);
    let user_service = UserService::new(user_repo);

    match GetUserByIdUseCase
        .execute(&user_service, pool.get_ref(), id.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(ApiResponse {
            message: Some("User fetched successfully".to_string()),
            success: true,
            data: user,
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            message: Some("Failed to fetch user".to_string()),
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

pub async fn update_user_route(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    request: web::Json<CreateUserRequest>,
) -> impl Responder {
    let user_repo = Arc::new(SqlxUserRepositoryImpl);
    let user_service = UserService::new(user_repo);

    match UpdateUserUseCase
        .execute(
            &user_service,
            pool.get_ref(),
            (id.into_inner(), request.into_inner()),
        )
        .await
    {
        Ok(user) => HttpResponse::Ok().json(ApiResponse {
            message: Some("User updated successfully".to_string()),
            success: true,
            data: Some(user),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            message: Some("Failed to update user".to_string()),
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

pub async fn delete_user_route(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    let user_repo = Arc::new(SqlxUserRepositoryImpl);
    let user_service = UserService::new(user_repo);

    match DeleteUserUseCase
        .execute(&user_service, pool.get_ref(), id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            message: Some("User deleted successfully".to_string()),
            success: true,
            data: None,
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            message: Some("Failed to delete user".to_string()),
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}
