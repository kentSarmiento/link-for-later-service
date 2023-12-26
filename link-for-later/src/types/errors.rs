use std::{error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum App {
    ServerError,
    DatabaseError(String),
    LinkNotFound,
    UserAlreadyExists,
    UserNotFound,
    IncorrectPassword,
    AuthorizationError(String),
    InvalidEmail,
    InvalidUrl,
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ServerError => write!(f, "server error"),
            Self::DatabaseError(_) => write!(f, "database error"),
            Self::LinkNotFound => write!(f, "link item not found"),
            Self::UserAlreadyExists => write!(f, "user already regisered"),
            Self::UserNotFound => write!(f, "user not found"),
            Self::IncorrectPassword => write!(f, "incorrect password for user"),
            Self::AuthorizationError(_) => write!(f, "invalid authorization token"),
            Self::InvalidEmail => write!(f, "invalid email"),
            Self::InvalidUrl => write!(f, "invalid url"),
        }
    }
}

impl error::Error for App {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_error_messages() {
        assert_eq!(App::ServerError.to_string(), "server error");
        assert_eq!(
            App::DatabaseError("a database error occurred".into()).to_string(),
            "database error"
        );
        assert_eq!(App::ServerError.to_string(), "server error");
        assert_eq!(App::LinkNotFound.to_string(), "link item not found");
        assert_eq!(App::UserAlreadyExists.to_string(), "user already regisered");
        assert_eq!(App::UserNotFound.to_string(), "user not found");
        assert_eq!(App::InvalidEmail.to_string(), "invalid email");
        assert_eq!(App::InvalidUrl.to_string(), "invalid url");
        assert_eq!(
            App::IncorrectPassword.to_string(),
            "incorrect password for user"
        );
        assert_eq!(
            App::AuthorizationError("authorization error occurred".into()).to_string(),
            "invalid authorization token"
        );
    }
}
