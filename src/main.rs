use std::env;
use warp::Filter;
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    // let port =  
    // println!("{}", port);
    //format!("The number is {}", 1);
    let port = env::var("PORT")
        .map(|port| port.parse::<i32>().unwrap_or(4000))
        .unwrap_or(4000);
    let server_details = format!("127.0.0.1:{}",port);
    let server: SocketAddr = server_details
    .parse()
    .expect("Unable to parse socket address");

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