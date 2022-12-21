use jubako::Server;

mod test_window;
use test_window::TestWindowCreator;

#[tokio::main]
async fn main() {
    let server = Server::new()
        .route_simple_window("/test-window", TestWindowCreator);

    println!("Open http://localhost:8080/test-window/ in your browser.");
    server.run(8080).await;
}
