use std::fmt::Display;

use salvo::{http::body, prelude::*};

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[handler]
async fn hello2(req: &mut Request, res: &mut Response) {
    let url = req.uri().to_string();
    let body = req.parse_json::<Body1>().await.unwrap().to_string();
    // let body = req.parse_json::<String>().await.unwrap().to_string();
    dbg!(body.as_str());
    let headers = req
        .headers()
        .iter()
        .map(|(k, v)| {
            format!(
                "key:{} = {}",
                k.as_str(),
                String::from_utf8_lossy(v.as_bytes())
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    res.render(Json(Response1 { url, body, headers }));
}

#[derive(serde::Serialize)]
struct Response1 {
    url: String,
    body: String,
    headers: String,
}
#[derive(serde::Deserialize)]
struct Body1 {
    name: String,
}

impl Display for Body1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}", self.name)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(Router::with_path("get").get(hello))
        .push(Router::with_path("post").post(hello2));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
