pub const AUTHCODE_FILE_PATH: &str = "./.authcode";

pub const GOOGLE_AUTH_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const GOOGLE_TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

pub const GOOGLE_REDIRECT_URI: &str = "http://localhost:8000/oauth2/google";
pub const GOOGLE_SCOPE: &str = "https://www.googleapis.com/auth/drive.metadata.readonly";
pub const GOOGLE_OAUTH_CLIENT_ID: &str = dotenv!("GOOGLE_OAUTH_CLIENT_ID");
pub const GOOGLE_OAUTH_CLIENT_SECRET: &str = dotenv!("GOOGLE_OAUTH_CLIENT_SECRET");
