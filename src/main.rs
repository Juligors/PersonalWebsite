use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use webella::app::*;

#[tokio::main]
async fn main() {

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options.clone();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // NOTE: Heroku has dynamic port inside env var PORT, if it's not there we use localhost setup from Dockerfile
    let addr = match std::env::var("PORT") {
        Ok(port) => format!("0.0.0.0:{}", port),
        Err(_) => conf.leptos_options.site_addr.to_string(),
    };

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
