use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
 
    use async_openai::config::OpenAIConfig;
    use serde::{Serialize, Deserialize};
    use secrecy::Secret;
    use dotenv::dotenv;

    pub const KEY_ENV_NAME: &str = "OPENAI_API_KEY";
    pub const ORG_ENV_NAME: &str = "OPENAI_ORGANIZATION";
    pub const NOT_FOUND: &str = "not found";
    /// Default v1 API base url
    pub const OPENAI_API_BASE: &str = "https://api.openai.com/v1";
    /// Name for organization header
    pub const OPENAI_ORGANIZATION_HEADER: &str = "OpenAI-Organization";

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ChatConfig {
        pub temperature: f32,
        pub top_p:f32,
        pub max_tokens: u16,
        pub model: String,
        pub stream: bool,
        pub presence_penalty: f32,
        pub n: u8,
    }

    /// Configuration for OpenAI API
    #[derive(Clone, Debug, Deserialize)]
    #[serde(default)]
    pub struct ConfigOpenAI {
        api_base: String,
        api_key: Secret<String>,
        org_id: String,
    }

    // OpenAI doesn't pull the org_id from the env
    // this will fully populate if env vars exist
    // otherwise they are empty, a simple test without throwing errors
    // and potentially exposing a secret
    impl Default for ConfigOpenAI {
        fn default() -> Self {
            Self {
                api_base: OPENAI_API_BASE.to_string(),
                // make it secret
                api_key: Secret::new(std::env::var("OPENAI_API_KEY")
                    .unwrap_or_else(|_| "".to_string())
                    .into()),
                // this is public on openai
                org_id: std::env::var("OPENAI_ORG_ID")
                .unwrap_or_else(|_| "".to_string())
                .into(),
            }
        }
    }

    impl ConfigOpenAI {
        // Create client with default [OPENAI_API_BASE] url
        // default API key from OPENAI_API_KEY env var and
        // default org_id from OPENAI_ORG_ID
        pub fn new() -> Self {
            Default::default()
        }
    }

    // Get API key and Org ID from environment variables
    pub async fn set_openai_config() -> Result<ConfigOpenAI, leptos::ServerFnError> {
      dotenv().ok();
      Ok(ConfigOpenAI::new())
    }

 
  }
}