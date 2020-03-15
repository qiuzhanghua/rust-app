use std::collections::HashSet;
use time::PrimitiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use actix_web::http::HeaderValue;
use jsonwebtoken::{DecodingKey, Validation, EncodingKey, Header};
use crate::auth::jwt::MemToken;
use std::hash::Hasher;

pub mod jwt;
pub mod handlers;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupVo {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginVo {
    name: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    exp: i64,
    iat: i64,
    name: String,
    email: String,
    login_extra: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBody {
    pub token: String,
    pub token_type: String,
}

impl std::cmp::PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl std::cmp::Eq for Account {}

impl std::hash::Hash for Account {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.name.as_ref())
    }
}

pub trait AuthService {
    fn signup(&mut self, sv: SignupVo) -> Result<String, String>;
    fn login(&mut self, lv: LoginVo) -> Result<String, String>;
    fn logout(&mut self, authen_header: &HeaderValue) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct AccountService {
    accounts: HashSet<Account>,
    secret: String,
}

impl AccountService {
    pub fn new() -> AccountService {
        let secret = viperus::get::<String>("auth.secret").unwrap();
        AccountService {
            accounts: HashSet::new(),
            secret,
        }
    }
}


impl AuthService for AccountService {
    fn signup(&mut self, sv: SignupVo) -> Result<String, String> {
        if self.accounts.iter().any(|x| { x.name == sv.name }) {
            Err(format!("User '{}' is already registered", &sv.name))
        } else {
            let now = PrimitiveDateTime::now().timestamp();
            let account = Account {
                exp: now + 3600 * 24,  // one day
                iat: now,
                name: sv.name,
                email: sv.email,
                login_extra: Uuid::new_v4().to_simple().to_string(),
            };
            self.accounts.insert(account.clone());
//            Ok(json!({ "token": account.login_extra, "token_type": "bearer" }).to_string())
            Ok(jsonwebtoken::encode(&Header::default(), &account, &EncodingKey::from_secret("secret".as_ref())).unwrap())
//            Ok(serde_json::to_string(&account).unwrap())
        }
    }

    fn login(&mut self, lv: LoginVo) -> Result<String, String> {
        if lv.name != lv.password {  // dummy
            Err("password error".to_string())
        } else {
            if let Some(x) = self.accounts.iter().find(|&x| { x.name == lv.name }) {
                // should update iat and exp etc
                // Ok(json!({ "token": x.login_extra, "token_type": "bearer" }).to_string())
                Ok(jsonwebtoken::encode(&Header::default(), x, &EncodingKey::from_secret("secret".as_ref())).unwrap())
            } else {
                Err("not found".to_string())
            }
        }
    }

    fn logout(&mut self, authen_header: &HeaderValue) -> Result<(), String> {
        info!("authen_header = {:?}", authen_header);
        if let Ok(authen_str) = authen_header.to_str() {
            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
//                jsonwebtoken::decode::<MemToken>(&token, decoding_key, &Validation::default());
                if let Ok(token_data) = jsonwebtoken::decode::<Account>(&token, &DecodingKey::from_secret(self.secret.as_ref()), &Validation::default()) {
                    let token = token_data.claims;
                    println!("token = {:?}", token);
                    let mut accounts = self.accounts.clone();
                    let account = accounts.iter().find(|&x| { x.name == token.name });
                    println!("account = {:?}", account);
                    if account.is_some() {
                        println!("logout OK");
                        self.accounts.remove(account.unwrap()); }
                    return Ok(());
                };
            };
        };
        Err("Logout".to_string())
    }
}