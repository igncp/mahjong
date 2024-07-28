use crate::env::ENV_AUTH_JWT_SECRET_KEY;
use crate::{
    env::{ENV_FRONTEND_URL, ENV_GITHUB_CLIENT_ID, ENV_GITHUB_SECRET},
    http_server::DataStorage,
    time::get_timestamp,
};
use actix_web::{HttpRequest, HttpResponse};
use argon2::{self, Config};
pub use github::{GithubAuth, GithubCallbackQuery};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mahjong_core::PlayerId;
use serde::{Deserialize, Serialize};
use service_contracts::{AuthInfoSummary, AuthProvider, ServicePlayer, UserPostSetAuthResponse};
use tracing::{debug, error};
use ts_rs::TS;
use uuid::Uuid;

pub use self::errors::{AuthInfoSummaryError, UnauthorizedError};

mod errors;
mod github;

pub type Username = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GetAuthInfo {
    EmailUsername(Username),
    AnonymousToken(String),
    GithubUsername(Username),
    PlayerId(PlayerId),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TS)]
#[ts(export)]
pub enum UserRole {
    Admin,
    Player,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthInfoGithub {
    pub id: PlayerId,
    pub token: Option<String>,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthInfoEmail {
    pub hashed_pass: String,
    pub id: PlayerId,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthInfoAnonymous {
    pub hashed_token: String,
    pub id: PlayerId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AuthInfoData {
    Anonymous(AuthInfoAnonymous),
    Email(AuthInfoEmail),
    Github(AuthInfoGithub),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthInfo {
    pub data: AuthInfoData,
    pub role: UserRole,
    pub user_id: PlayerId,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
struct TokenClaims {
    exp: usize,
    role: UserRole,
    sub: String,
}

pub struct AuthHandler<'a> {
    auth_info: Option<AuthInfo>,
    req: &'a HttpRequest,
    storage: &'a DataStorage,
}

impl<'a> AuthHandler<'a> {
    pub fn verify_setup() -> bool {
        std::env::var(ENV_AUTH_JWT_SECRET_KEY).is_ok()
            && std::env::var(ENV_GITHUB_CLIENT_ID).is_ok()
            && std::env::var(ENV_GITHUB_SECRET).is_ok()
            && std::env::var(ENV_FRONTEND_URL).is_ok()
    }

    pub fn new(storage: &'a DataStorage, req: &'a HttpRequest) -> Self {
        Self {
            auth_info: None,
            req,
            storage,
        }
    }

    pub async fn validate_email_user(
        &mut self,
        username: &String,
        password: &String,
    ) -> Result<Option<bool>, String> {
        let auth_info_opts = GetAuthInfo::EmailUsername(username.clone());
        let auth_info = self.storage.get_auth_info(auth_info_opts).await?;

        if auth_info.is_none() {
            debug!("Not found auth_info for username: {username}");
            return Ok(None);
        }

        let auth_info_content = auth_info.unwrap();
        let auth_info_email = auth_info_content.data.clone();

        let auth_info_email = match auth_info_email {
            AuthInfoData::Email(email) => email,
            _ => {
                debug!("Unexpected auth_info for username: {username}");
                return Ok(None);
            }
        };

        let hash = auth_info_email.hashed_pass.clone();
        let matches = argon2::verify_encoded(&hash, password.as_bytes());

        if matches.is_err() {
            let err_str = matches.err().unwrap().to_string();
            debug!("Matches produced an error for username: {username}, error: {err_str}");
            return Err(err_str);
        }

        let matches = matches.unwrap();

        self.auth_info = Some(auth_info_content);

        Ok(Some(matches))
    }

    pub async fn validate_anon_user(
        &mut self,
        id_token: &String,
    ) -> Result<Option<bool>, UnauthorizedError> {
        let salt = Uuid::new_v4().to_string();
        let config = Config::default();
        let hashed_token = argon2::hash_encoded(id_token.as_bytes(), salt.as_bytes(), &config)
            .map_err(|_| {
                error!("Error hashing token");
                UnauthorizedError
            })?;

        let auth_info_opts = GetAuthInfo::AnonymousToken(hashed_token.clone());
        let auth_info = self
            .storage
            .get_auth_info(auth_info_opts)
            .await
            .map_err(|_| UnauthorizedError)?;

        if auth_info.is_none() {
            debug!("Not found auth_info for id_token: {id_token}");
            return Ok(None);
        }

        let auth_info_content = auth_info.unwrap();
        let auth_info_anonymous = auth_info_content.data.clone();

        if let AuthInfoData::Anonymous(_) = auth_info_anonymous {
            Ok(Some(true))
        } else {
            Ok(None)
        }
    }

    pub async fn create_email_user(
        &mut self,
        username: &Username,
        password: &String,
        role: UserRole,
    ) -> Result<(), String> {
        let salt = Uuid::new_v4().to_string();
        let config = Config::default();
        let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
        let user_id = Uuid::new_v4().to_string();

        let auth_info_email = AuthInfoEmail {
            hashed_pass: hash.clone(),
            id: user_id.clone(),
            username: username.clone(),
        };

        let player = ServicePlayer {
            id: user_id.clone(),
            name: username.clone(),
            created_at: get_timestamp().to_string(),

            ..ServicePlayer::default()
        };

        self.storage.save_player(&player).await?;

        let auth_info = AuthInfo {
            data: AuthInfoData::Email(auth_info_email),
            role,
            user_id,
        };

        self.storage.save_auth_info(&auth_info).await?;

        self.auth_info = Some(auth_info);

        Ok(())
    }

    pub async fn create_anonymous_user(
        &mut self,
        token: &String,
        role: UserRole,
    ) -> Result<(), String> {
        let salt = Uuid::new_v4().to_string();
        let config = Config::default();
        let hash = argon2::hash_encoded(token.as_bytes(), salt.as_bytes(), &config).unwrap();
        let user_id = Uuid::new_v4().to_string();

        let auth_info_anonymous = AuthInfoAnonymous {
            hashed_token: hash.clone(),
            id: user_id.clone(),
        };

        let random_suffix = Uuid::new_v4().to_string().replace('-', "")[0..6].to_string();
        let name = format!("Anonymous User {}", random_suffix);

        let player = ServicePlayer {
            id: user_id.clone(),
            name,
            created_at: get_timestamp().to_string(),

            ..ServicePlayer::default()
        };

        self.storage.save_player(&player).await?;

        let auth_info = AuthInfo {
            data: AuthInfoData::Anonymous(auth_info_anonymous),
            role,
            user_id,
        };

        self.storage.save_auth_info(&auth_info).await?;

        self.auth_info = Some(auth_info);

        Ok(())
    }

    pub async fn create_github_user(
        &mut self,
        username: &Username,
        token: &str,
        role: UserRole,
    ) -> Result<(), String> {
        let user_id = Uuid::new_v4().to_string();

        let auth_info_github = AuthInfoGithub {
            id: user_id.clone(),
            token: Some(token.to_string()),
            username: username.clone(),
        };

        let player = ServicePlayer {
            id: user_id.clone(),
            name: "Github user".to_string(),
            created_at: get_timestamp().to_string(),

            ..ServicePlayer::default()
        };

        self.storage.save_player(&player).await?;

        let auth_info = AuthInfo {
            data: AuthInfoData::Github(auth_info_github),
            role,
            user_id,
        };

        self.storage.save_auth_info(&auth_info).await?;

        self.auth_info = Some(auth_info);

        Ok(())
    }

    pub fn generate_token(&self) -> Result<UserPostSetAuthResponse, String> {
        if self.auth_info.is_none() {
            debug!("Tried to generate token but no user is logged in");
            return Err("No user logged in".to_string());
        }

        let auth_info = self.auth_info.as_ref().unwrap();
        let my_claims = TokenClaims {
            exp: 9999999999,
            role: auth_info.role.clone(),
            sub: auth_info.user_id.clone(),
        };

        let encoding_secret = std::env::var(ENV_AUTH_JWT_SECRET_KEY);

        if encoding_secret.is_err() {
            return Err("Error decoding".to_string());
        }

        let encoding_secret = encoding_secret.unwrap();

        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(encoding_secret.as_ref()),
        )
        .unwrap();

        let response = UserPostSetAuthResponse { token };

        Ok(response)
    }

    fn get_token_claims(&self, outer_token: Option<&String>) -> Option<TokenClaims> {
        let encoding_secret = std::env::var(ENV_AUTH_JWT_SECRET_KEY);

        if encoding_secret.is_err() {
            error!("Missing encoding_secret environment variable");
            return None;
        }

        let encoding_secret = encoding_secret.unwrap();

        let token = if let Some(outer_token) = outer_token {
            outer_token.clone()
        } else {
            let authorization = self.req.headers().get("authorization");

            authorization?;

            let authorization = authorization.unwrap().to_str();

            if authorization.is_err() {
                return None;
            }

            let authorization = authorization.unwrap();

            authorization.replace("Bearer ", "")
        };

        let token_message = decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(&encoding_secret.into_bytes()),
            &Validation::new(Algorithm::HS256),
        );

        if token_message.is_err() {
            return None;
        }

        let token_message = token_message.unwrap();

        Some(token_message.claims)
    }

    fn get_verify_user_claims(claims: Option<TokenClaims>, player_id: &PlayerId) -> bool {
        if claims.is_none() {
            debug!("No claims for player_id: {player_id}");
            return false;
        }

        let claims = claims.unwrap();
        claims.sub == *player_id
    }

    fn get_verify_admin_claims(claims: Option<TokenClaims>) -> bool {
        if claims.is_none() {
            return false;
        }

        let claims = claims.unwrap();

        claims.role == UserRole::Admin
    }

    pub fn verify_user(&self, player_id: &PlayerId) -> Result<(), UnauthorizedError> {
        let claims = self.get_token_claims(None);

        let is_user = AuthHandler::get_verify_user_claims(claims, player_id);

        if !is_user {
            return Err(UnauthorizedError);
        }

        Ok(())
    }

    pub fn get_user_from_token(&self) -> Result<String, UnauthorizedError> {
        let claims = self.get_token_claims(None);

        if claims.is_none() {
            return Err(UnauthorizedError);
        }

        Ok(claims.unwrap().sub)
    }

    pub fn verify_user_token(&self, player_id: &PlayerId, token: &String) -> bool {
        let claims = self.get_token_claims(Some(token));

        AuthHandler::get_verify_user_claims(claims, player_id)
    }

    pub fn verify_admin(&self) -> Result<(), UnauthorizedError> {
        let claims = self.get_token_claims(None);

        let is_admin = AuthHandler::get_verify_admin_claims(claims);

        if !is_admin {
            return Err(UnauthorizedError);
        }

        Ok(())
    }

    pub fn verify_admin_token(&self, token: &String) -> bool {
        let claims = self.get_token_claims(Some(token));

        AuthHandler::get_verify_admin_claims(claims)
    }

    pub fn get_unauthorized() -> HttpResponse {
        HttpResponse::Unauthorized().body("Unauthorized")
    }

    pub async fn get_auth_info_summary(&self) -> Result<AuthInfoSummary, AuthInfoSummaryError> {
        let user_id = self
            .get_user_from_token()
            .map_err(|_| AuthInfoSummaryError::Unauthorized)?;

        let user = self
            .storage
            .get_auth_info(GetAuthInfo::PlayerId(user_id.clone()))
            .await;

        if user.is_err() {
            return Err(AuthInfoSummaryError::DatabaseError);
        }

        let user = user
            .unwrap()
            .map_or_else(|| Err(AuthInfoSummaryError::DatabaseError), Ok)?;

        let summary = match user.data {
            AuthInfoData::Anonymous(_) => AuthInfoSummary {
                provider: AuthProvider::Anonymous,
                username: None,
            },
            AuthInfoData::Email(email) => AuthInfoSummary {
                provider: AuthProvider::Email,
                username: Some(email.username),
            },
            AuthInfoData::Github(github) => AuthInfoSummary {
                provider: AuthProvider::Github,
                username: Some(github.username),
            },
        };

        Ok(summary)
    }
}
