use jwt_simple::prelude::*;

use crate::models::user::User;

const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISSUER: &str = "chat_server";
const JWT_AUDIENCE: &str = "chat_web";

#[derive(Clone)]
pub struct EncodingKey(Ed25519KeyPair);

#[derive(Clone)]
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn load(key: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519KeyPair::from_pem(key)?))
    }

    pub fn sign(&self, user: User) -> Result<String, jwt_simple::Error> {
        let claims = Claims::with_custom_claims(user, Duration::from_secs(JWT_DURATION))
            .with_issuer(JWT_ISSUER)
            .with_audience(JWT_AUDIENCE);

        self.0.sign(claims)
    }
}

impl DecodingKey {
    pub fn load(key: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519PublicKey::from_pem(key)?))
    }

    pub fn verify(&self, token: &str) -> Result<User, jwt_simple::Error> {
        let options = VerificationOptions {
            allowed_issuers: Some(HashSet::from([JWT_ISSUER.to_string()])),
            allowed_audiences: Some(HashSet::from([JWT_AUDIENCE.to_string()])),
            ..Default::default()
        };

        let claims = self.0.verify_token::<User>(token, Some(options))?;

        Ok(claims.custom)
    }
}

impl std::fmt::Debug for EncodingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncodingKey").finish()
    }
}

impl std::fmt::Debug for DecodingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecodingKey").finish()
    }
}
