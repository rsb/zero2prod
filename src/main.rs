use zero2prod::run;
use tokio::net::TcpListener;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8000");
  // Bubble up the io::Error if we fail to bind the address
  // Otherwise call .await on our server
  run(listener)?.await
}
