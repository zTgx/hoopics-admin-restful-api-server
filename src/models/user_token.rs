use crate::models::entry;
use chrono::Utc;
use jsonwebtoken::{
    EncodingKey,
    DecodingKey,
    Validation,
    Algorithm,

    Header
};

pub static KEY: [u8; 16] = *include_bytes!("../../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    // pub login_session: String,
}

impl UserToken {
    pub async fn generate_token(login: &entry::AdminInfo) -> String { 
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.user_id.clone(),
        };

        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }

    pub fn decode_token(token: &String) -> Result<UserToken, String> {
        match jsonwebtoken::decode::<UserToken>(token, &DecodingKey::from_secret(&KEY), &Validation::new(Algorithm::HS256)) {
            Ok(user_token) => Ok(user_token.claims),
            Err(_) => Err("api-token 已过期或者错误".to_string()) 
        }
    }
}