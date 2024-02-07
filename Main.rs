

*Advanced Rust Project: Implementing a Concurrent Web Server*

rust
// Advanced Rust program: Concurrent web server using Actix web framework

use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use std::sync::{Arc, Mutex};

// Shared state between multiple threads
struct AppState {
    counter: Arc<Mutex<usize>>,
}

async fn index(data: web::Data<AppState>, _req: HttpRequest) -> impl Responder {
    // Increment counter
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    // Display counter value
    HttpResponse::Ok().body(format!("Counter: {}", *counter))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Shared state for multiple threads
    let data = web::Data::new(AppState {
        counter: Arc::new(Mutex::new(0)),
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // Share state with all routes
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


*Rust Code: Concurrent Web Server*

- *Input*: No direct user input is provided. The input is an HTTP request sent to the web server, typically through a web browser or a tool like cURL. For example:
  
  curl http://localhost:8080/
  

- *Output*: When the root endpoint is accessed, the server responds with an HTTP OK status code (200) and a message displaying the current value of the counter. For example:
  
  Counter: 1
  

- *Explanation*: Each time the root endpoint is accessed, the counter increments by one and the updated counter value is displayed in the response body.


