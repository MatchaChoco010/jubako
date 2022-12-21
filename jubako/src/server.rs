//! Server that host the gui application.
//!
//! # Example

use axum::{
    body::{boxed, Full},
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Path,
    },
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use rust_embed::RustEmbed;

use crate::simple_window;
use crate::Message;

async fn handle_static(Path(path): Path<String>) -> impl IntoResponse {
    StaticFile(path)
}

async fn handle_simple_window_socket(socket: WebSocket, runner: simple_window::SimpleWindowRunner) {
    runner.run(socket).await;
}

async fn not_found() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(boxed(Full::from("404")))
        .unwrap()
}

pub struct Server {
    app: Router,
}
impl Server {
    /// Create a new jubako server.
    pub fn new() -> Self {
        Self { app: Router::new() }
    }

    /// Add route simple window.
    pub fn route_simple_window<T: Message>(
        self,
        path: &str,
        window_creator: impl simple_window::SimpleWindowCreator<Message = T>,
    ) -> Self {
        let path = String::from(path).trim_end_matches('/').to_string();
        if path == "" {
            panic!("path must not be empty");
        }

        // handle index.html
        let app = self
            .app
            .route(
                &format!("{path}/"),
                get(|| async { StaticFile("index.html") }),
            )
            .route(
                &format!("{path}/index.html"),
                get(|| async { StaticFile("index.html") }),
            );

        // handle static files
        let app = app.route(&format!("{path}/*path"), get(handle_static));

        // handle websocket
        let app = app.route(
            &format!("{path}/ws"),
            get(|ws: WebSocketUpgrade| async move {
                let runner = simple_window::SimpleWindowRunner::new(window_creator);
                ws.on_upgrade(|websocket| handle_simple_window_socket(websocket, runner))
            }),
        );

        Self { app }
    }

    /// Start run the server.
    pub async fn run(self, port: u16) {
        axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
            .serve(
                self.app
                    .fallback_service(get(not_found))
                    .into_make_service(),
            )
            .await
            .unwrap();
    }
}

#[derive(RustEmbed)]
#[folder = "assets/dist/"]
struct Asset;

pub struct StaticFile<T>(pub T);
impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}
