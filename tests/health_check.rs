//! tests/health_check.rs

use std::net::TcpListener;

// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[actix_rt::test]
async fn health_check_works() {
  // Arrange
  let address  = spawn_app();
  let client = reqwest::Client::new();

  // Act
  let response = client
    .get(&format!("{}/health_check", address))
    .send()
    .await
    .expect("Failed to execute");

  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

// No .await call, therefore no need for spawn_app to be async now.
// We are also running tests so its not worth propagating errors:
// If we fail to perform the proper setup we can just panic
fn spawn_app()  -> String {
  let listener = TcpListener::bind("127:0.0.1:0")
    .expect("Failed to bind to random port");

  let server = zero2prod::run(listener).expect("failed to bind address");

  // Launch the server as a background task
  // tokio::spawn returns a handle to the spawned future,
  // but we have no use for it here, hence the non-binding let
  //
  // New dev dependency - let's add tokio to the party with
  // `cargo add tokio --dev --vers 1`
  let _ = tokio::spawn(server);
  let port = listener.local_addr().unwrap().port();

  format!("http://127.0.0.1:{}", port)
}