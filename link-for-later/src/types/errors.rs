use std::{error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum App {
    ServerError(String),
    DatabaseError(String),
    LinkNotFound(String),
    UserAlreadyExists(String),
    UserNotFound(String),
    IncorrectPassword(String),
    AuthorizationError(String),
    ValidationError(String),

    #[cfg(test)]
    TestError,
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ServerError(_) => write!(f, "server error"),
            Self::DatabaseError(_) => write!(f, "database error"),
            Self::LinkNotFound(_) => write!(f, "link item not found"),
            Self::UserAlreadyExists(_) => write!(f, "user already registered"),
            Self::UserNotFound(_) => write!(f, "user not found"),
            Self::IncorrectPassword(_) => write!(f, "incorrect password for user"),
            Self::AuthorizationError(_) => write!(f, "invalid authorization token"),
            Self::ValidationError(_) => write!(f, "invalid request"),

            #[cfg(test)]
            Self::TestError => write!(f, "test error"),
        }
    }
}

impl error::Error for App {}

#[cfg(test)]
mod tests {

    use super::App as AppError;

    #[test]
    fn test_error_messages() {
        assert_eq!(
            AppError::ServerError("a server operation failed".into()).to_string(),
            "server error"
        );
        assert_eq!(
            AppError::DatabaseError("a database operation failed".into()).to_string(),
            "database error"
        );
        assert_eq!(
            AppError::LinkNotFound("link".into()).to_string(),
            "link item not found"
        );
        assert_eq!(
            AppError::UserAlreadyExists("user".into()).to_string(),
            "user already registered"
        );
        assert_eq!(
            AppError::UserNotFound("user".into()).to_string(),
            "user not found"
        );
        assert_eq!(
            AppError::IncorrectPassword("user".into()).to_string(),
            "incorrect password for user"
        );
        assert_eq!(
            AppError::AuthorizationError("an authorization error occurred".into()).to_string(),
            "invalid authorization token"
        );
        assert_eq!(
            AppError::ValidationError("invalid email".into()).to_string(),
            "invalid request"
        );
    }
}
