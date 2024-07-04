use std::env;
use std::sync::OnceLock;

use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use regex::Regex;
use tera::Tera;

use crate::error::Result;

// pub const DATE_FORMAT_LONG: &str = "%A %d %B %Y";
// pub const DATE_FORMAT_SHORT: &str = "%a %d %b %Y";

pub struct Config {
    pub database_url: String,
    pub domain: String,
    pub auth_cookie_name: String,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_host: String,
    pub smtp_port: String,
    pub sendgrid_api_key: String,
    pub tera: Tera,
    pub from_address: String,
    pub template_account_activation: String,
    pub template_password_reset: String,
    pub template_email_change: String,
    pub token_duration_authentication: i64,
    pub token_duration_account_activation: i64,
    pub token_duration_password_reset: i64,
    pub token_duration_email_change: i64,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub slug_regex: Regex,
    pub email_regex: Regex,
    pub access_code: String,
}

impl Config {
    fn new() -> Result<Self> {
        dotenv().ok();

        let template_dir = env::var("TEMPLATE_DIR").unwrap();
        let mut tera = Tera::new(&template_dir).expect("Uable to load template dir");

        tera.autoescape_on(vec![".html", ".sql"]);

        let secret_key = env::var("SECRET_KEY").unwrap();
        let secret = secret_key.as_bytes();

        let email_regex = Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
        let slug_regex = Regex::new(r"[^a-z0-9]+").expect("valid regex");

        let config = Config {
            domain: env::var("DOMAIN").unwrap(),
            database_url: env::var("DATABASE_URL").unwrap(),
            auth_cookie_name: env::var("AUTH_COOKIE_NAME").unwrap(),
            access_code: env::var("ACCESS_CODE").unwrap(),
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),

            smtp_host: env::var("SMTP_HOST").unwrap(),
            smtp_user: env::var("SMTP_USER").unwrap(),
            smtp_pass: env::var("SMTP_PASS").unwrap(),
            smtp_port: env::var("SMTP_PORT").unwrap(),

            sendgrid_api_key: env::var("SENDGRID_API_KEY").unwrap(),
            tera,
            from_address: env::var("FROM_ADDRESS").unwrap(),

            template_account_activation: env::var("TEMPLATE_ACCOUNT_ACTIVATION").unwrap(),
            template_password_reset: env::var("TEMPLATE_PASSWORD_RESET").unwrap(),
            template_email_change: env::var("TEMPLATE_EMAIL_CHANGE").unwrap(),

            token_duration_authentication: env::var("TOKEN_DURATION_AUTHENTICATION")
                .unwrap()
                .parse()
                .unwrap(),
            token_duration_account_activation: env::var("TOKEN_DURATION_ACCOUNT_ACTIVATION")
                .unwrap()
                .parse()
                .unwrap(),
            token_duration_password_reset: env::var("TOKEN_DURATION_PASSWORD_RESET")
                .unwrap()
                .parse()
                .unwrap(),
            token_duration_email_change: env::var("TOKEN_DURATION_EMAIL_CHANGE")
                .unwrap()
                .parse()
                .unwrap(),

            email_regex,
            slug_regex,
        };

        Ok(config)
    }
}

pub fn get_config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| Config::new().expect("Error loading env variables"))
}
