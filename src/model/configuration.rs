use cfg_if::cfg_if;

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AIModel {
  Gpt4,
  Gpt35Turbo,
  Bloom,
  Gpt2,
  GptJ,
  GptNeoX,
  #[default]
  Llama,
  Mpt,
}

impl std::str::FromStr for AIModel {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.to_lowercase().as_str() {
          "gpt4" => Ok(AIModel::Gpt4),
          "gpt35turbo" => Ok(AIModel::Gpt35Turbo),
          "bloom" => Ok(AIModel::Bloom),
          "gpt2" => Ok(AIModel::Gpt2),
          "gptj" => Ok(AIModel::GptJ),
          "gptneox" => Ok(AIModel::GptNeoX),
          "llama" => Ok(AIModel::Llama),
          "mpt" => Ok(AIModel::Mpt),
          "" => Ok(AIModel::default()), // Return default for empty string
          _ => Err(()), // Return an error for any other string
      }
  }
}


cfg_if! {
    if #[cfg(feature = "ssr")] {

      use env_logger;

      fn load_module(model: AIModel) {
        match model {
            AIModel::Gpt4 => {
                // Load module for Gpt4
                
                log::info!("Loading GPT-4 module");
            },
            AIModel::Gpt35Turbo => {
                // Load module for Gpt35Turbo
                log::info!("Loading GPT-3.5 Turbo module");
            },
            AIModel::Bloom => {
                // Load module for Bloom
                log::info!("Loading BLOOM module");
            },
            AIModel::Gpt2 => {
                // Load module for Gpt2
                log::info!("Loading GPT-2 module");
            },
            AIModel::GptJ => {
                // Load module for GptJ
                log::info!("Loading GPT-J module");
            },
            AIModel::GptNeoX => {
                // Load module for GptNeoX
                log::info!("Loading GPT-NeoX module");
            },
            AIModel::Llama => {
                // Load module for Llama
                log::info!("Loading LLAMA module");
            },
            AIModel::Mpt => {
                // Load module for Mpt
                log::info!("Loading MPT module");
            },
        }
    }
    
  }
}