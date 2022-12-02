use rust_web_server_yew::http::{
    error::Error,
    request::{request_get, request_post},
};

use crate::types::{
    auth::{SigninInfoWrapper, SignupInfoWrapper, UserInfoWrapper},
    EmptyWrapper,
};

/// Get current user info
pub async fn current() -> Result<UserInfoWrapper, Error> {
    request_get::<UserInfoWrapper>("/user".to_string()).await
}

/// Signin a user
pub async fn signin(login_info: SigninInfoWrapper) -> Result<UserInfoWrapper, Error> {
    request_post::<SigninInfoWrapper, UserInfoWrapper>("/users/login".to_string(), login_info).await
}

/// Signup a new user
pub async fn signup(signup_info: SignupInfoWrapper) -> Result<EmptyWrapper, Error> {
    request_post::<SignupInfoWrapper, EmptyWrapper>("/users".to_string(), signup_info).await
}
