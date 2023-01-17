use std::{net::SocketAddr, env};

use axum::{Router, routing::get, extract::Path};

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    let _ = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(val) => val,
        Err(_) => panic!("TELEGRAM_BOT_TOKEN not set"),
    };
    let _ = match env::var("TELEGRAM_CHAT_ID") {
        Ok(val) => val,
        Err(_) => panic!("TELEGRAM_CHAT_ID not set"),
    };

    env_logger::init();
    let app = Router::new()
        .route("/push/:message", get(push_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn push_handler(Path(message): Path<String>) -> &'static str {
    let result = send_telegram_message(&message).await;
    match result {
        Ok(_) => "OK",
        Err(_) => "ERROR",
    }
}

async fn send_telegram_message(message: &String) -> Result<(), Box<dyn std::error::Error>> {
    let bot_token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(val) => val,
        Err(_) => panic!("TELEGRAM_BOT_TOKEN not set"),
    };
    let chart_id = match env::var("TELEGRAM_CHAT_ID") {
        Ok(val) => val,
        Err(_) => panic!("TELEGRAM_CHAT_ID not set"),
    };
    let client = reqwest::Client::new();
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let _ = client
        .post(url)
        .json(&serde_json::json!({
            "chat_id": chart_id,
            "text": message,
        }))
        .send()
        .await
        .unwrap();
    Ok(())
}
