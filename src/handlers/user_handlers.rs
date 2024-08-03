use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::models::user::User;

#[derive(Deserialize)]
pub struct UserId {
    pub id: String,
}

pub async fn get_users() -> HttpResponse {
    HttpResponse::Ok().json(vec![
        User {
            id: 1,
            name: "John Doe".to_string(),
        },
        User {
            id: 2,
            name: "Jane Doe".to_string(),
        },
    ])
}

pub async fn get_user(user_id: web::Path<UserId>) -> HttpResponse {
    let user = User {
        id: user_id.id.parse().unwrap(),
        name: "John Doe".to_string(),
    };
    HttpResponse::Ok().json(user)
}

pub async fn create_user(user: web::Json<User>) -> HttpResponse {
    HttpResponse::Created().json(user.into_inner())
}
