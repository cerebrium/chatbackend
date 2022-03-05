use warp::Filter;

const WEB_FOLDER: &str = "web/";

#[tokio::main]
async fn main() {
    // API endpoints
    let welcome_route = warp::path::end()
        .and(warp::get())
        .map(|| "Hello, world!");

    let hi = warp::path("hi").and(warp::get())
        .map(|| "Hi!");

        
    // Static
    let content = warp::fs::dir(WEB_FOLDER);
    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}index.html", WEB_FOLDER)));
        
    let static_files = content.or(root);
        
    let routes = welcome_route.or(hi).or(static_files);

    println!("starting web server");
    warp::serve(routes).run(([127, 0, 0, 1], 3001)).await;
}
