use axum::{extract::State, http::StatusCode, response::IntoResponse, routing, Json, Router};
use validator::Validate;

use crate::types::{
    AppError, AppState, UserInfoBuilder, UserLoginRequest, UserLoginResponse, UserRegisterRequest,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest(
            "/v1",
            Router::new().nest(
                "/users",
                Router::new()
                    .route("/login", routing::post(login))
                    .route("/register", routing::post(register)),
            ),
        )
        .with_state(state)
}

async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<UserRegisterRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(()) => {}
        Err(e) => {
            return AppError::Validation(format!("register() {e:?}")).into_response();
        }
    }

    let users_repo = app_state.users_repo().clone();
    let user_info = UserInfoBuilder::new(payload.email(), payload.password())
        .admin(payload.admin())
        .build();
    match app_state
        .users_service()
        .register(Box::new(users_repo), &user_info)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => e.into_response(),
    }
}

async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<UserLoginRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(()) => {}
        Err(e) => {
            return AppError::Validation(format!("login() {e:?}")).into_response();
        }
    }

    let users_repo = app_state.users_repo().clone();
    let user_info = UserInfoBuilder::new(payload.email(), payload.password()).build();
    match app_state
        .users_service()
        .login(Box::new(users_repo), &user_info)
        .await
    {
        Ok(token) => {
            let response = UserLoginResponse::new(token.jwt());
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => e.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http_body_util::BodyExt;
    use rstest::rstest;
    use serde_json::json;

    use crate::{
        repository::{MockLinks as MockLinksRepo, MockUsers as MockUsersRepo},
        service::DynUsers as DynUsersService,
        service::{
            MockAnalysis as MockAnalysisService, MockLinks as MockLinksService,
            MockUsers as MockUsersService,
        },
        types::Token,
    };

    use super::*;

    #[rstest]
    #[tokio::test]
    async fn test_register_user(#[values(true, false)] is_admin: bool) {
        let request = UserRegisterRequest::new("user@test.com", "test", is_admin);
        let user_to_register = UserInfoBuilder::new("user@test.com", "test")
            .admin(is_admin)
            .build();
        let registered_user = UserInfoBuilder::new("user@test.com", "test")
            .admin(is_admin)
            .build();

        let mut mock_users_service = MockUsersService::new();
        mock_users_service
            .expect_register()
            .withf(move |_, user| user == &user_to_register)
            .times(1)
            .returning(move |_, _| Ok(registered_user.clone()));

        let app_state = AppStateBuilder::new(Arc::new(mock_users_service)).build();
        let response = register(State(app_state), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::CREATED, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"");
    }

    #[rstest]
    #[tokio::test]
    async fn test_register_user_invalid_email(#[values(true, false)] is_admin: bool) {
        let request = UserRegisterRequest::new("user", "test", is_admin);

        let mut mock_users_service = MockUsersService::new();
        mock_users_service.expect_register().times(0);

        let app_state = AppStateBuilder::new(Arc::new(mock_users_service)).build();
        let response = register(State(app_state), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::BAD_REQUEST, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "invalid request"}).to_string());
    }

    #[rstest]
    #[tokio::test]
    async fn test_register_user_service_error(#[values(true, false)] is_admin: bool) {
        let request = UserRegisterRequest::new("user@test.com", "test", is_admin);
        let user_to_register = UserInfoBuilder::new("user@test.com", "test")
            .admin(is_admin)
            .build();

        let mut mock_users_service = MockUsersService::new();
        mock_users_service
            .expect_register()
            .withf(move |_, user| user == &user_to_register)
            .times(1)
            .returning(|_, _| Err(AppError::Test));

        let app_state = AppStateBuilder::new(Arc::new(mock_users_service)).build();
        let response = register(State(app_state), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "test error"}).to_string());
    }

    #[tokio::test]
    async fn test_login_user() {
        let request = UserLoginRequest::new("user@test.com", "test");
        let user_to_login = UserInfoBuilder::new("user@test.com", "test").build();
        let token = Token::new("test");

        let mut mock_users_service = MockUsersService::new();
        mock_users_service
            .expect_login()
            .withf(move |_, user| user == &user_to_login)
            .times(1)
            .returning(move |_, _| Ok(token.clone()));

        let app_state = AppStateBuilder::new(Arc::new(mock_users_service)).build();
        let response = login(State(app_state), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::OK, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"token": "test"}).to_string());
    }

    #[tokio::test]
    async fn test_login_user_invalid_email() {
        let request = UserLoginRequest::new("user", "test");

        let mut mock_users_service = MockUsersService::new();
        mock_users_service.expect_login().times(0);

        let app_state = AppStateBuilder::new(Arc::new(mock_users_service)).build();
        let response = login(State(app_state), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::BAD_REQUEST, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "invalid request"}).to_string());
    }

    #[tokio::test]
    async fn test_login_user_service_error() {
        let request = UserLoginRequest::new("user@test.com", "test");
        let user_to_login = UserInfoBuilder::new("user@test.com", "test").build();

        let mut mock_users_service = MockUsersService::new();
        mock_users_service
            .expect_login()
            .withf(move |_, user| user == &user_to_login)
            .times(1)
            .returning(|_, _| Err(AppError::Test));

        let app_state = AppStateBuilder::new(Arc::new(mock_users_service)).build();
        let response = login(State(app_state), Json(request)).await;

        let (parts, body) = response.into_response().into_parts();
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, parts.status);

        let body = body.collect().await.unwrap().to_bytes();
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, json!({"error": "test error"}).to_string());
    }

    struct AppStateBuilder {
        users_service: DynUsersService,
    }

    impl AppStateBuilder {
        fn new(users_service: DynUsersService) -> Self {
            Self { users_service }
        }

        fn build(self) -> AppState {
            AppState::new(
                Arc::new(MockLinksService::new()),
                self.users_service,
                Arc::new(MockAnalysisService::new()),
                Arc::new(MockLinksRepo::new()),
                Arc::new(MockUsersRepo::new()),
            )
        }
    }
}
