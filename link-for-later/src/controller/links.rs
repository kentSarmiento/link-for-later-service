use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Json, Router,
};

use crate::{
    state::AppState,
    types::{
        auth::Claims,
        dto::{LinkItemRequest, LinkQueryBuilder},
        entity::LinkItemBuilder,
    },
};

const LINKS_ROUTE: &str = "/v1/links";
const LINKS_ID_ROUTE: &str = "/v1/links/:id";

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(LINKS_ROUTE, routing::get(list))
        .route(LINKS_ROUTE, routing::post(post))
        .route(LINKS_ID_ROUTE, routing::get(get))
        .route(LINKS_ID_ROUTE, routing::put(put))
        .route(LINKS_ID_ROUTE, routing::delete(delete))
        .with_state(state)
}

async fn list(State(app_state): State<AppState>, user: Claims) -> impl IntoResponse {
    let link_query = LinkQueryBuilder::default().owner(user.id()).build();
    match app_state
        .links_service()
        .search(Box::new(app_state.links_repo().clone()), &link_query)
        .await
    {
        Ok(list) => Json(list).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn post(
    State(app_state): State<AppState>,
    user: Claims,
    Json(payload): extract::Json<LinkItemRequest>,
) -> impl IntoResponse {
    let link_item = LinkItemBuilder::default()
        .owner(user.id())
        .url(payload.url())
        .title(payload.title())
        .description(payload.description())
        .build();
    match app_state
        .links_service()
        .create(Box::new(app_state.links_repo().clone()), &link_item)
        .await
    {
        Ok(link) => (StatusCode::CREATED, Json(link)).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn get(
    State(app_state): State<AppState>,
    user: Claims,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let link_query = LinkQueryBuilder::new(&id, user.id()).build();
    match app_state
        .links_service()
        .get(Box::new(app_state.links_repo().clone()), &link_query)
        .await
    {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn put(
    State(app_state): State<AppState>,
    user: Claims,
    Path(id): Path<String>,
    Json(payload): extract::Json<LinkItemRequest>,
) -> impl IntoResponse {
    let link_item = LinkItemBuilder::new(&id, user.id())
        .url(payload.url())
        .title(payload.title())
        .description(payload.description())
        .build();
    match app_state
        .links_service()
        .update(Box::new(app_state.links_repo().clone()), &id, &link_item)
        .await
    {
        Ok(link) => Json(link).into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}

async fn delete(
    State(app_state): State<AppState>,
    user: Claims,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let link_item = LinkItemBuilder::new(&id, user.id()).build();
    match app_state
        .links_service()
        .delete(Box::new(app_state.links_repo().clone()), &link_item)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("Error: {}", e);
            e.into_response()
        }
    }
}
