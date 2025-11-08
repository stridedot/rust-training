use std::io::Read;

use base64::prelude::*;
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::SECRET_KEY_LENGTH;
use ed25519_dalek::SIGNATURE_LENGTH;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::{Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::utils;
use crate::{opts::text::TextSignFormat, process::genpass};

pub async fn process_generate(format: &TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate().await,
        TextSignFormat::Ed25519 => Ed25519Signer::generate().await,
    }
}

pub async fn process_sign(
    input: &str,
    key: &str,
    format: &TextSignFormat,
) -> anyhow::Result<String> {
    let mut reader = utils::get_reader(input).await?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };

    let signed = BASE64_URL_SAFE_NO_PAD.encode(&signed);

    Ok(signed)
}

pub async fn process_verify(
    input: &str,
    key: &str,
    sig: &str,
    format: &TextSignFormat,
) -> anyhow::Result<bool> {
    let mut reader = utils::get_reader(input).await?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let sig = BASE64_URL_SAFE_NO_PAD.decode(sig)?;

    let is_valid = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };

    Ok(is_valid)
}

trait GenerateKey {
    async fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

trait TextSign {
    fn sign<T: Read>(&self, reader: &mut T) -> anyhow::Result<Vec<u8>>;
}

trait TextVerify {
    fn verify<T: Read>(&self, reader: &mut T, sig: &[u8]) -> anyhow::Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

impl GenerateKey for Blake3 {
    async fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = genpass::process_genpass(32, true, true, true, true).await?;
        Ok(vec![key.into_bytes()])
    }
}

impl TextSign for Blake3 {
    fn sign<T: Read>(&self, reader: &mut T) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let hash = blake3::keyed_hash(&self.key, &buf);

        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<T: Read>(&self, reader: &mut T, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let hash = blake3::keyed_hash(&self.key, &buf);

        Ok(hash.as_bytes() == sig)
    }
}

impl Blake3 {
    fn load(key: &str) -> anyhow::Result<Self> {
        let key = std::fs::read(key)?;
        Self::try_from(key)
    }

    fn try_from(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = &key[..32];
        let signer = Blake3 {
            key: key.try_into()?,
        };

        Ok(signer)
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl GenerateKey for Ed25519Signer {
    async fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key: VerifyingKey = signing_key.verifying_key();

        Ok(vec![
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        ])
    }
}

impl TextSign for Ed25519Signer {
    fn sign<T: Read>(&self, reader: &mut T) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);

        Ok(sig.to_bytes().to_vec())
    }
}

impl Ed25519Signer {
    fn load(key: &str) -> anyhow::Result<Self> {
        let key = std::fs::read(key)?;
        Self::try_from(key)
    }

    fn try_from(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = &key[..SECRET_KEY_LENGTH].try_into()?;
        let signer = Ed25519Signer {
            key: SigningKey::from_bytes(key),
        };

        Ok(signer)
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextVerify for Ed25519Verifier {
    fn verify<T: Read>(&self, reader: &mut T, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = Signature::from_bytes(&sig[..SIGNATURE_LENGTH].try_into()?);
        let result = self.key.verify(&buf, &sig);

        Ok(result.is_ok())
    }
}

impl Ed25519Verifier {
    fn load(key: &str) -> anyhow::Result<Self> {
        let key = std::fs::read(key)?;
        Self::try_from(key)
    }

    fn try_from(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = &key[..PUBLIC_KEY_LENGTH].try_into()?;
        let verifier = Ed25519Verifier {
            key: VerifyingKey::from_bytes(key)?,
        };

        Ok(verifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blake3_sign_verify() -> anyhow::Result<()> {
        let key = Blake3::generate().await?;
        let signer = Blake3::try_from(&key[0])?;
        let verifier = Blake3::try_from(&key[0])?;

        let input = "hello world".as_bytes();
        let mut reader = input;
        let sig = signer.sign(&mut reader)?;

        let mut reader = input;
        let is_valid = verifier.verify(&mut reader, &sig)?;

        assert!(is_valid);

        Ok(())
    }

    #[tokio::test]
    async fn test_ed25519_sign_verify() -> anyhow::Result<()> {
        let key = Ed25519Signer::generate().await?;
        let signer = Ed25519Signer::try_from(&key[0])?;
        let verifier = Ed25519Verifier::try_from(&key[1])?;

        let input = "hello world".as_bytes();
        let mut reader = input;
        let sig = signer.sign(&mut reader)?;

        let mut reader = input;
        let is_valid = verifier.verify(&mut reader, &sig)?;

        assert!(is_valid);

        Ok(())
    }
}
