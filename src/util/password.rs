use anyhow::{anyhow, Context};
use argon2::password_hash::SaltString;
use argon2::{password_hash, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use tokio::task;

pub async fn hash(password: String) -> anyhow::Result<String> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::new(15000, 2, 1, None)
                .map_err(|e| anyhow!(e).context("failed to generate hash params"))?,
        )
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!(e).context("failed to hash password"))?
        .to_string())
    })
    .await?
}

pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow!(e).context("BUG: password hash invalid"))?;

        let res = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::new(15000, 2, 1, None)
                .map_err(|e| anyhow!(e).context("failed to generate hash params"))?,
        )
        .verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("failed to verify password")),
        }
    })
    .await?
}
