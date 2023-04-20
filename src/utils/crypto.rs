use actix_web::{web::block};
use argonautica::{Hasher, Verifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::{sync::Arc};
use uuid::Uuid;
use tracing::{instrument, event, Level};
#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>,
    pub jwt_secret: Arc<String>
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64
}

#[derive(Serialize)]

pub struct Auth {
    pub token: String,
}

impl CryptoService {
    pub async fn hash_password(&self, password: String) -> Result<String, Box<dyn std::error::Error>> {
        let key = self.key.clone();       
        let hash = block( move || {
        Hasher::default()
        .with_secret_key(&*key)
        .with_password(password)
        .hash()
        .unwrap()
        
       }).await;
       
       match hash {
        Ok(hash) => Ok(hash),
        Err(err) => {
           event!(Level::ERROR, "Hashing password: {}", err);
           Err(Box::new(err))
       }
       
        }

    }

    pub async fn verify_password(
        &self,
        password: String,
        pass_hash: String
    ) -> Result<bool, ()> {
        let key = self.key.clone();

        let verify = block( move || {
            Verifier::default()
            .with_secret_key(&*key)
            .with_hash(pass_hash)
            .with_password(password)
            .verify()
            .unwrap()
        }).await;

        match verify {
            Ok(verify) => Ok(verify),
            Err(err) => {
                event!(Level::ERROR, "Verifying password: {}", err);
                Err(())
            }
        }

    }
    
    #[instrument(skip(self))]
    pub async fn gen_jwt(&self, user_id: Uuid) -> Result<Result<String, Box<dyn std::error::Error>>, Box<dyn std::error::Error>> {
        let jwt_key = self.jwt_secret.clone();
        let generaotor = block(move || {
            let headers = Header::default();
            let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
            let now = Utc::now() + Duration::days(1); // Token Expires in 1 day
            let claims = Claims {
                sub: user_id,
                exp: now.timestamp()
            };
            encode(&headers, &claims, &encoding_key)
            .unwrap()
        })
        .await;

        match generaotor {
            Ok(generaotor) => Ok(Ok(generaotor)),
            Err(err) => {
                event!(Level::ERROR, "Error genratoring a JWT token for user {} with err {}", user_id, err);
                Err(Box::new(err))
            }
        }

    }

    pub async fn verify_jwt(&self, token: String) -> Result<Result<TokenData<Claims>, Box<dyn std::error::Error>>, ()> {
        let jwt_key = self.jwt_secret.clone();
        let verify = block( move || {
            let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
            let validation = Validation::default();
            decode::<Claims>(&token, &decoding_key, &validation)
            .unwrap()
        }) .await;

        match verify {
            Ok(verify) => Ok(Ok(verify)),
            Err(err) => {
                event!(Level::ERROR, "error verifing jwt token, err: {}", err);
                Err(())
            }
        }

    }
}
