use warp::Filter;

#[tokio::main]
async fn main() {
    let welcome_route = warp::path::end().map(|| "Hello, world!");

    println!("starting web server");
    warp::serve(welcome_route).run(([127, 0, 0, 1], 3001)).await;
}
