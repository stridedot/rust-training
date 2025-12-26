use chrono::{DateTime, Utc};
use jwt_simple::prelude::*;
use tonic::{Request, Status, service::Interceptor};

#[derive(Clone)]
pub struct DecodingKey(Ed25519PublicKey);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub workspace_id: i64,
    pub created_at: DateTime<Utc>,
}

// const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISSUER: &str = "chat_server";
const JWT_AUDIENCE: &str = "chat_web";

impl DecodingKey {
    pub fn load(key: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519PublicKey::from_pem(key.trim())?))
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

impl Interceptor for DecodingKey {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let token = req
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok());

        let user = match token {
            Some(token) => {
                let token = token
                    .strip_prefix("Bearer ")
                    .ok_or_else(|| Status::unauthenticated("invalid token format"))?;

                self.verify(token)
                    .map_err(|e| Status::unauthenticated(format!("{:?}", e)))?
            }
            None => return Err(Status::unauthenticated("missing token")),
        };

        req.extensions_mut().insert(user);

        Ok(req)
    }
}
