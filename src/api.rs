use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        static LLAMA_SESSION: &str = "config/llama_session.yaml";
        static LLAMA_MESSAGE: &str = "config/llama_messages.yaml";
                
        use std::sync::Arc;
        use std::convert::Infallible;
        use std::fs;
        use actix_web::web;
        use actix_web::HttpRequest;
        use actix_web::HttpResponse;
        use actix_web::web::Payload;
        use actix_web::Error;
        use actix_ws::Message as Msg;
        use llm::models::Llama;
        use llm::KnownModel;
        use futures::stream::{StreamExt};
        use leptos::*;
        use env_logger;
        
        use crate::model::conversation::{Message, Role};
        
        pub fn infer(model: Arc<Llama>, inference_session: &mut llm::InferenceSession, user_message: Message, tx: tokio::sync::mpsc::Sender<String>) -> Result<(), ServerFnError> {
            use tokio::runtime::Runtime;
            use llm::InferenceFeedback::Halt;

            // would love a way to avoid doing this if possible
            let mut runtime = Runtime::new().expect("issue creating tokio runtime");

            let asst = Role::Assistant.to_llama();
            let role = user_message.role.to_llama();
            let msg = user_message.content;

            if msg == "" {
                
            }
            
            inference_session
                .infer(
                    model.as_ref(),
                    &mut rand::thread_rng(),
                    &llm::InferenceRequest {
                        prompt: format!("{role}\n{msg}\n{asst}:")
                            .as_str()
                            .into(),
                        parameters: &llm::InferenceParameters::default(),
                        play_back_previous_tokens: false,
                        maximum_token_count: Some(256),
                    },
                    &mut Default::default(),
                    inference_callback(String::from(role), &mut String::new(), tx, &mut runtime),
                )
                .unwrap_or_else(|e| panic!("{e}"));

            Ok(())
        }

        fn session_setup(model: Arc<Llama>) -> llm::InferenceSession {     
            let file_content = fs::read_to_string(LLAMA_MESSAGE).expect("Cannot read file");
            let messages = serde_yaml::from_str::<Vec<Message>>(&file_content).expect("No Messages");

            let mut persona = String::new();
            let mut history = String::new();
        
            for message in &messages {
                match message.role {
                    Role::System | Role::Assistant => {
                        persona += message.content.clone().as_str();
                    }
                    _ => {
                        // Replacing placeholder and appending to history
                        history.push_str(&format!("{}: {}\n", message.role.to_llama(), message.content));
                    }
                }
            }
        
            log::info!("Persona:\n{}", persona);
            log::info!("History:\n{}", history);

            let mut session = model.start_session(Default::default());
            session
                .feed_prompt(
                    model.as_ref(),
                    format!("{persona}\n{history}").as_str(),
                    &mut Default::default(),
                    llm::feed_prompt_callback(|_| {
                        Ok::<llm::InferenceFeedback, Infallible>(llm::InferenceFeedback::Continue)
                    }),
                )
                .expect("Failed to ingest initial prompt.");

            session
        }

        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            tx: tokio::sync::mpsc::Sender<String>,
            runtime: &'a mut tokio::runtime::Runtime,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;

            move |resp| -> Result<llm::InferenceFeedback, Infallible> {match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    // Clone the string we're going to send
                    let text_to_send = if buf.is_empty() {
                        t.clone()
                    } else {
                        reverse_buf
                    };

                    let tx_cloned = tx.clone();
                    runtime.block_on(async move {
                        tx_cloned.send(text_to_send).await.expect("issue sending on channel");
                    });

                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }}
        }

        pub async fn ws(req: HttpRequest, body: Payload, model: web::Data<Llama>) -> Result<HttpResponse, Error> {
            let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

            use std::sync::Mutex;
            use tokio::sync::mpsc;


            let (send_inference, mut recieve_inference) = mpsc::channel(100);

            let mdl: Arc<Llama> = model.into_inner().clone();
            let sess = Arc::new(Mutex::new(session));
            let sess_cloned = sess.clone();
            actix_rt::spawn(async move {
                let (send_new_user_message, recieve_new_user_message) =
                    std::sync::mpsc::channel();

                // Rustformers sessions need to stay on the same thread
                // So we can't really rely on TOKIOOOOO
                let model_cloned = mdl.clone();
                // let send_inference_cloned = send_inference.clone();

                std::thread::spawn(move || {
                    let mut inference_session = session_setup(mdl);

                    for new_user_message in recieve_new_user_message {
                        let msg = Message::new(Some(Role::User), new_user_message);
                        if msg.content == "" {
                            //end conversation
                            break
                        }
                        log::info!("Received New User Message:{}", msg.content);
                        let _ = infer(model_cloned.clone(), &mut inference_session, msg, send_inference.clone());
                    }
                });

                while let Some(Ok(msg)) = msg_stream.next().await {
                    match msg {
                        Msg::Ping(bytes) => {
                            let res = sess_cloned.lock().unwrap().pong(&bytes).await;
                            if res.is_err() {
                                return;
                            }
                        }
                        Msg::Text(s) => {
                            // send it to the dedicated inference thread
                            let _ = send_new_user_message.send(s.to_string());
                        }
                        _ => break,
                    }
                }
            });

            // another task to receive inferred tokens and send them
            // over the WebSocket connection while the inference
            // itself chugs away on a separate thread
            actix_rt::spawn(async move {
                while let Some(message) = recieve_inference.recv().await {
                    sess.lock().unwrap().text(message).await.expect("issue sending on websocket");
                }
                // let _ = sess.lock().unwrap().close(None).await;
            });

            Ok(response)
        }
    }
}
