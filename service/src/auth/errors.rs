use std::fmt::Display;

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub struct UnauthorizedError;
#[derive(Debug)]
pub enum AuthInfoSummaryError {
    DatabaseError,
    Unauthorized,
}

impl Display for UnauthorizedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unauthorized")
    }
}

impl ResponseError for UnauthorizedError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

impl Display for AuthInfoSummaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError => write!(f, "Database error"),
            Self::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl ResponseError for AuthInfoSummaryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::DatabaseError => HttpResponse::InternalServerError().body("Database error"),
            Self::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
        }
    }
}
