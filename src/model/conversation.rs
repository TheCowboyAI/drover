use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
  System,
  #[default]
  User,
  Assistant,
  Tool,
  Function,
}

impl Role {
  pub fn to_llama(&self) -> &'static str {
      match self {
          Role::System => "### Assistant",
          Role::User => "### User",
          Role::Assistant => "### Assistant",
          Role::Tool => "### Tool",
          Role::Function => "### Function",
      }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Message {
    pub role: Role, 
    pub content: String,
}

impl Message {
    pub fn new(role: Option<Role>, content: String) -> Message {
        Message {
            role: role.unwrap_or_default(), // Use provided role or default to User
            content: content.to_owned(),
        }
    }

    pub fn is_user(&self) -> bool {
        self.role == Role::User
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {

        use tokio::fs::{write, read_to_string}; 
        use std::error::Error;
        
        // example usage
        // let date = Utc::now();
        // let file_path = format!("chat.{}.yml", date.format("%Y-%m-%d[%H:%M:%S]"));

        // Serialize and write messages to a YAML file
        pub async fn save_to_yaml(messages: Vec<Message>, file_path: &str) -> Result<(), Box<dyn Error>> {
            let yaml_str = serde_yaml::to_string(&messages)?;
            write(file_path, yaml_str).await?;
            Ok(())
        }

        // Read YAML from a file and deserialize into a Conversation instance
        pub async fn from_yaml(file_path: &str) -> Result<Conversation, Box<dyn Error>> {
            let yaml_str = read_to_string(file_path).await?;
            let messages: Vec<Message> = serde_yaml::from_str(&yaml_str)?;
            Ok(Conversation { messages })
        }
    }
}

