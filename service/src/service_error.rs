use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ServiceError {
    GameNotFound,
    GameVersionMismatch,
    ErrorLoadingGame,
    Custom(&'static str),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameNotFound => write!(f, "Game not found"),
            Self::GameVersionMismatch => write!(f, "Game version mismatch"),
            Self::ErrorLoadingGame => write!(f, "Error loading game"),
            Self::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::GameNotFound => StatusCode::NOT_FOUND,
            Self::GameVersionMismatch => StatusCode::BAD_REQUEST,
            Self::ErrorLoadingGame => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Custom(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::GameNotFound => HttpResponse::NotFound().body("Game not found"),
            Self::GameVersionMismatch => HttpResponse::BadRequest().body("Game version mismatch"),
            Self::ErrorLoadingGame => {
                HttpResponse::InternalServerError().body("Error loading game")
            }
            Self::Custom(msg) => HttpResponse::InternalServerError().body(msg.to_string()),
        }
    }
}

pub type ResponseCommon = actix_web::Result<HttpResponse>;
