use cfg_if::cfg_if;

pub mod api;
pub mod model;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // debug logging, disable for prod
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use cim_web::app::*;
    use cim_web::model::llama::get_language_model;
    use api::ws;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|| view! { <App/> });

    #[get("/style.css")]
    async fn css() -> impl Responder {
        actix_files::NamedFile::open_async("./style/output.css").await
    }

    let ai_model = get_AI_Model();
    

    let model = web::Data::new(get_language_model());

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .app_data(model.clone())
            .service(css)
            .route("/ws", web::get().to(ws))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! { <App/> },
            )
            .service(Files::new("/", site_root))
    })
    .bind(&addr)?
    .run()
    .await
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        // determine the AI model we are talking to

        use actix_web::*;
        use std::env;
        use dotenv::dotenv;
        use crate::model::configuration::AIModel;
        use std::str::FromStr;
        
        fn get_AI_Model() -> AIModel {
            dotenv().ok();

            // Attempt to read the MODEL environment variable
            let aimodel_str = env::var("MODEL").unwrap_or_else(|_| "".to_string());

            // Convert the string to an AIModel enum, 
            // defaulting if the string is empty
            AIModel::from_str(&aimodel_str).unwrap_or_else(|_| AIModel::default())
        }       
    }
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `ssg` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features ssg`
    use leptos::*;
    use cim_web::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use api::ws;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move || {
        // note: for testing it may be preferrable to replace this with a
        // more specific component, although leptos_router should still work
        view! {<App/> }
    });
}
