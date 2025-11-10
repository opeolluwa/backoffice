use std::{env, str::FromStr};

use crate::errors::Error;

pub fn extract_env<T: FromStr>(env_key: &str) -> Result<T, Error> {
    let env = env::var(env_key)
        .map_err(|_| Error::EnvError(env_key.to_string()))?
        .parse::<T>()
        .map_err(|_| Error::EnvError("error parsing env".into()))?;

    Ok(env)
}
