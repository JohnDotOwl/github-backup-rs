use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, BackupError>;

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("api error: {0}")]
    Api(#[from] ApiError),

    #[error("git error: {0}")]
    Git(#[from] GitError),

    #[error("authentication error: {0}")]
    Auth(#[from] AuthError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("repository '{repo}' unavailable (DMCA); legal URL: {legal_url}")]
    RepositoryUnavailable { repo: String, legal_url: String },

    #[error("not implemented yet: {0}")]
    Unimplemented(&'static str),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("http client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("unexpected response status {status}: {message}")]
    UnexpectedStatus { status: StatusCode, message: String },

    #[error("request failed after retries: {0}")]
    RetriesExhausted(String),
}

impl ApiError {
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http(error) => {
                error.is_timeout() || error.is_connect() || error.is_request() || error.is_body()
            }
            Self::UnexpectedStatus { status, .. } => matches!(
                *status,
                StatusCode::TOO_MANY_REQUESTS
                    | StatusCode::FORBIDDEN
                    | StatusCode::BAD_GATEWAY
                    | StatusCode::SERVICE_UNAVAILABLE
                    | StatusCode::GATEWAY_TIMEOUT
            ),
            Self::RetriesExhausted(_) => true,
        }
    }
}

#[derive(Debug, Error)]
pub enum GitError {
    #[error("git command failed: {command} (status: {status:?})\n{stderr}")]
    CommandFailed {
        command: String,
        status: Option<i32>,
        stderr: String,
    },

    #[error("git io error: {source}")]
    Io {
        #[source]
        source: std::io::Error,
    },

    #[error("invalid git url: {0}")]
    InvalidUrl(String),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("missing authentication token")]
    MissingToken,

    #[error("invalid authentication configuration: {0}")]
    InvalidConfig(String),

    #[error("failed reading token file '{path}': {source}")]
    TokenFileRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed reading keychain token: {0}")]
    Keychain(String),
}
