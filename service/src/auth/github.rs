use actix_web::web::Query;
use serde::{Deserialize, Serialize};
use service_contracts::UserPostSetAuthResponse;

use crate::{
    auth::{GetAuthInfo, UserRole},
    env::{ENV_GITHUB_CLIENT_ID, ENV_GITHUB_SECRET},
    http_server::DataStorage,
};

use super::AuthHandler;

#[derive(Deserialize, Debug)]
pub struct GithubCallbackQuery {
    code: String,
}

pub struct GithubAuth {}

impl GithubAuth {
    pub async fn handle_callback(
        query: Query<GithubCallbackQuery>,
        storage: &DataStorage,
        auth_handler: &mut AuthHandler<'_>,
    ) -> Option<UserPostSetAuthResponse> {
        #[derive(Serialize, Debug)]
        struct GithubAccessBody {
            client_id: String,
            client_secret: String,
            code: String,
        }

        let body = GithubAccessBody {
            client_id: std::env::var(ENV_GITHUB_CLIENT_ID).unwrap(),
            client_secret: std::env::var(ENV_GITHUB_SECRET).unwrap(),
            code: query.code.clone(),
        };

        let url = "https://github.com/login/oauth/access_token";
        let json_data = serde_json::to_string(&body).unwrap();

        let client = reqwest::Client::new();

        let response2 = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(json_data.to_owned())
            .send()
            .await
            .unwrap();

        let response_body_text2 = response2.text().await.unwrap();

        #[derive(Deserialize, Debug)]
        struct GithubAccessResponse {
            access_token: String,
        }

        let response_body2 =
            serde_qs::from_str::<GithubAccessResponse>(&response_body_text2).unwrap();

        let access_token = response_body2.access_token.clone();
        let response = client
            .get("https://api.github.com/user")
            .header("Content-Type", "application/json")
            .header("User-Agent", "Rust Server")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .unwrap();
        let response_body_text = response.text().await.unwrap();

        #[derive(Deserialize, Debug)]
        struct GithubUserResponse {
            login: String,
        }

        let response_body3 =
            serde_json::from_str::<GithubUserResponse>(&response_body_text).unwrap();

        let existing_user = storage
            .get_auth_info(GetAuthInfo::GithubUsername(response_body3.login.clone()))
            .await
            .unwrap();

        if existing_user.is_none() {
            let result = auth_handler
                .create_github_user(&response_body3.login, &access_token, UserRole::Player)
                .await;

            if result.is_err() {
                return None;
            }
        } else {
            auth_handler.auth_info = existing_user;
        }

        let data = auth_handler.generate_token();

        if data.is_err() {
            return None;
        }

        Some(data.unwrap())
    }
}
