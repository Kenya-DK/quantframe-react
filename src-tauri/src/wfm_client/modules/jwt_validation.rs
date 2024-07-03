use eyre::eyre;
use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::modules::{error::AppError, logger};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: i64,
    iat: i64,
    iss: String,
    aud: String,
    auth_type: String,
    secure: bool,
    login_ua: String,
    login_ip: String,
    jwt_identity: String,
}

pub fn jwt_is_valid(jwt: &str, logging_component: &str) -> Result<bool, AppError> {
    match validate_jwt(jwt, None) {
        Ok(_) => Ok(true),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => {
                logger::warning_con(
                    format!("{}:{}", logging_component, "JWTValidation").as_str(),
                    "Token is invalid",
                );
                return Ok(false)
            },
            ErrorKind::InvalidIssuer => {
                logger::warning_con(
                    format!("{}:{}", logging_component, "JWTValidation").as_str(),
                    "Issuer is invalid",
                );
                return Ok(false)
            },
            ErrorKind::InvalidAudience => {
                logger::warning_con(
                    format!("{}:{}", logging_component, "JWTValidation").as_str(),
                    "Audience is invalid",
                );
                return Ok(false)
            },
            ErrorKind::ExpiredSignature => {
                logger::warning_con(
                    format!("{}:{}", logging_component, "JWTValidation").as_str(),
                    "JWT Expired",
                );
                return Ok(false)
            },
            _ => return Err(AppError::new("JWTValidation", eyre!(err.to_string()))),
        }
    }
}

fn validate_jwt(token: &str, input_key: Option<&[u8]>) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = None;
    validation.set_audience(&["jwt"]);
    validation.set_issuer(&["jwt"]);
    validation.set_required_spec_claims(&["exp", "iss", "aud"]);


    // Not sure about this tbh... It should be fine since we're only validating jwt's
    // from a single site but its still probably not ideal. jsonwebtoken::decode_header()
    // does about the same thing but doesnt give the exact behavior I'm looking for,
    // so if anyone has a better solution to this, let me or kenya know. :3
    let key = if input_key.is_none() {
        validation.insecure_disable_signature_validation();
        DecodingKey::from_secret(&[])
    } else {
        DecodingKey::from_secret(input_key.unwrap())
    };

    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data)
}


#[cfg(test)]
mod tests {

    use chrono::Utc;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    use crate::wfm_client::modules::jwt_validation::{validate_jwt, Claims};

    // fn stress_test() -> io::Result<()> {
    //     let now_tot = SystemTime::now();
    //     for _ in 0..10000 {
    //         let now = SystemTime::now();
    //         test_validate_jwt()?;
    //         println!("subtime: {}s", now.elapsed().unwrap().as_secs_f32());
    //     }
    //     println!("Total time: {}s", now_tot.elapsed().unwrap().as_secs_f32());
    //     Ok(())
    // }
    #[test]
    fn test_valid_jwt() {
        let key = b"oooo sppokyu key fkjsdn";
        let mut header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());
        let sample_claims_valid = Claims {
            exp: 100000000000,
            iat: 1719973822,
            iss: "jwt".to_string(),
            aud: "jwt".to_string(),
            auth_type: "coookie".to_string(),
            secure: true,
            login_ua: "Rusty Rivens".to_string(),
            login_ip: "numbershaha".to_string(),
            jwt_identity: "hi".to_string(),
        };
        let input_valid = encode(&header, &sample_claims_valid, &EncodingKey::from_secret(key)).unwrap();
        let jwt_valid = validate_jwt(input_valid.as_str(), Some(key)).unwrap();
        let now = Utc::now().timestamp();
        // println!("{:?}", jwt.claims);
        // println!("{:?}", jwt.header);
        assert!(jwt_valid.claims.exp > now, "JWT is no longer valid");
    }

    #[test]
    fn test_valid_jwt_no_key() {
        let key = b"oooo sppokyu key fkjsdn";
        let mut header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());
        let sample_claims_valid = Claims {
            exp: 100000000000,
            iat: 1719973822,
            iss: "jwt".to_string(),
            aud: "jwt".to_string(),
            auth_type: "coookie".to_string(),
            secure: true,
            login_ua: "Rusty Rivens".to_string(),
            login_ip: "numbershaha".to_string(),
            jwt_identity: "hi".to_string(),
        };
        let input_valid = encode(&header, &sample_claims_valid, &EncodingKey::from_secret(key)).unwrap();
        let _jwt_no_key = validate_jwt(input_valid.as_str(), None).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_expired_jwt() {
        let key = b"oooo sppokyu key fkjsdn";
        let mut header = Header::new(Algorithm::HS256);
        header.typ = Some("JWT".to_string());
        let sample_claims_expired = Claims {
            exp: 1719974000,
            iat: 1719973822,
            iss: "jwt".to_string(),
            aud: "jwt".to_string(),
            auth_type: "coookie".to_string(),
            secure: true,
            login_ua: "Rusty Rivens".to_string(),
            login_ip: "numbershaha".to_string(),
            jwt_identity: "hi".to_string(),
        };
        let input_expired = encode(&header, &sample_claims_expired, &EncodingKey::from_secret(key)).unwrap();
        let jwt_expired = validate_jwt(input_expired.as_str(), Some(key));
        assert!(jwt_expired.is_err());
        jwt_expired.unwrap();
    }
}
