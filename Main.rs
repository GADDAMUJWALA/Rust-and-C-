

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


*Advanced C++ Project: Implementing a Parallel Image Processing Application*

cpp
// Advanced C++ program: Parallel image processing using OpenMP

#include <iostream>
#include <vector>
#include <opencv2/opencv.hpp>
#include <omp.h>

void process_image(cv::Mat& image) {
    // Process image (e.g., apply filter, resize, etc.)
    cv::GaussianBlur(image, image, cv::Size(5, 5), 0);
}

int main() {
    // Load image
    cv::Mat image = cv::imread("input.jpg");

    // Split image into smaller regions
    std::vector<cv::Rect> regions;
    // Populate 'regions' vector with subregions of 'image'

    // Process each subregion in parallel
#pragma omp parallel for
    for (size_t i = 0; i < regions.size(); ++i) {
        cv::Mat sub_image = image(regions[i]);
        process_image(sub_image);
    }

    // Merge processed subregions into final image
    // (This step may require synchronization if necessary)

    // Save final image
    cv::imwrite("output.jpg", image);

    return 0;
}

