use tokio::prelude::*;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};

// Define a struct to represent a mobile app
#[derive(Serialize, Deserialize)]
struct MobileApp {
    id: i32,
    name: String,
    version: String,
}

// Define a struct to represent a real-time message
#[derive(Serialize, Deserialize)]
struct RealtimeMessage {
    app_id: i32,
    message: String,
}

#[tokio::main]
async fn main() {
    // Create a Tokio runtime
    let mut rt = Builder::new_current_thread().enable_io().enable_time().build().unwrap();

    // Create a channel to communicate between the app integrator and the real-time messaging system
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Spawn a task to simulate a mobile app sending real-time messages
    rt.spawn(async move {
        let app = MobileApp {
            id: 1,
            name: "MyApp".to_string(),
            version: "1.0".to_string(),
        };

        tx.send(RealtimeMessage {
            app_id: app.id,
            message: "Hello, world!".to_string(),
        }).await.unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        tx.send(RealtimeMessage {
            app_id: app.id,
            message: "Goodbye, world!".to_string(),
        }).await.unwrap();
    });

    // Spawn a task to simulate the real-time messaging system
    rt.spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Received message from app {}: {}", msg.app_id, msg.message);
        }
    });

    // Run the Tokio runtime
    rt.shutdown_timeout(std::time::Duration::from_millis(500));
}