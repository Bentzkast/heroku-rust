use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .map(|port| port.parse().unwrap_or(4000))
        .unwrap_or(4000);
    let enviroment = env::var("ENVIRONMENT").unwrap_or("DEVELOPMENT".to_string());
    
    let server = match enviroment.as_str() {
        "PRODUCTION" => {
            println!("production");
            ([0, 0, 0, 0], port)
        },
        "DEVELOPMENT" | _ => {
            println!("development");
            ([127, 0, 0, 1], port)
        },
    } ;

    // GET /
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let router = hello.or(hello_world);

    println!("listening on {}", port);
    warp::serve(router)
        .run(server)
        .await;
}