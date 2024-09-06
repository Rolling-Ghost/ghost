use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

async fn retrieve(
    Path(id): Path<i32>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn add(
    State(state): State<MyState>,
    Json(data): Json<TodoNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("INSERT INTO todos (note) VALUES ($1) RETURNING id, note")
        .bind(&data.note)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn add_user(
    State(state): State<MyState>,
    Json(data): Json<UserNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username, password",
    )
    .bind(&data.username)
    .bind(&data.password)
    .fetch_one(&state.pool)
    .await
    {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

// Retrieve the user with the given username
async fn retrieve_user(
    Path(username): Path<String>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(&state.pool)
        .await
    {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = MyState { pool };
    let router = Router::new()
        .route("/todos", post(add))
        .route("/todos/:id", get(retrieve))
        .route("/user", post(add_user))
        .route("/user/:username", get(retrieve_user))
        .with_state(state);

    Ok(router.into())
}

#[derive(Deserialize)]
struct UserNew {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
}

#[derive(Serialize, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
}
