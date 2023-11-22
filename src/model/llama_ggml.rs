use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {

    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct LlamaConfig {
        pub temperature: f32,
        pub max_tokens: u16,
    }

  }
}