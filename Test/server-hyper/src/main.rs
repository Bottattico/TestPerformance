use std::net::SocketAddr;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response};
use tokio::net::TcpListener;

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Gestisci solo il metodo GET per il percorso "/"
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Hello\n",
        ))),

        // Se il metodo non è GET o il percorso non è "/"
        _ => Ok(Response::new(Body::from("Metodo non consentito"))),
    }
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

  let listener = TcpListener::bind(addr).await?;
  println!("Listening on http://{}", addr);
  loop {
    let (stream, _) = listener.accept().await?;

    tokio::task::spawn(async move {
        if let Err(err) = Http::new().serve_connection(stream, service_fn(echo)).await {
          println!("Error serving connection: {:?}", err);
        }
    });
  }
}
