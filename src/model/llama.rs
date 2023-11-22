use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {
    use llm::models::Llama;
    use std::env;
    use dotenv::dotenv;

    pub fn get_language_model() -> Llama {
      use llm::models::Llama;
      use std::path::PathBuf;
      dotenv().ok();
      let model_path = env::var("MODEL_PATH").expect("MODEL_PATH must be set");
      let model_parameters = llm::ModelParameters {
          prefer_mmap: true,
          context_size: 2048,
          lora_adapters: None,
          use_gpu: true,
          gpu_layers: None,
          rope_overrides: None,
          n_gqa: None,
      };

      llm::load::<Llama>(
          &PathBuf::from(&model_path),
          llm::TokenizerSource::Embedded,
          model_parameters,
          llm::load_progress_callback_stdout,
      )
      .unwrap_or_else(|err| {
          panic!("Failed to load model from {model_path:?}: {err}")
      })
    }
  }
}