use crate::{
    Result,
    Error,
};

#[derive(Debug, Clone, Copy)]
pub enum EnvironmentVariable {
    DailymotionClientId,
    DailymotionClientSecret,
}

impl EnvironmentVariable {
    #[must_use]
    pub fn get_env_var_key(&self) -> &'static str {
        match self {
            Self::DailymotionClientId => "DAILYMOTION_CLIENT_ID",
            Self::DailymotionClientSecret => "DAILYMOTION_CLIENT_SECRET",
        }
    }

    /// Tries to get the value of the environment variable.
    ///
    /// # Errors
    /// Returns an [`Error::MissingEnvironmentVariable`] if unset.
    pub fn get_value(&self) -> Result<String> {
        std::env::var(self.get_env_var_key())
            .map_err(|_| Error::MissingEnvironmentVariable(*self))
    }
}

impl std::fmt::Display for EnvironmentVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_env_var_key())
    }
}

