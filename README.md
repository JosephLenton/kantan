<div align="center">
  <h1>
    Kantan<br>
    a simple way to make requests to a server
  </h1>

  [![crate](https://img.shields.io/crates/v/kantan.svg)](https://crates.io/crates/kantan)
  [![docs](https://docs.rs/kantan/badge.svg)](https://docs.rs/kantan)
</div>

Kantan is for making requests to servers. Lots of libraries exist for that.
Why use this?

 * Comes with batteries included.
 * Can automatically save cookies from responses -- useful for logging in, and then making a followup request.

## Features

This is for spinning up an Axum service, that you can then query directly.
This is primarily for testing Axum services.

```rust
  use ::axum::Router;
  use ::axum::routing::get;

  use ::axum_test_server::Server;

  async fn get_ping() -> &'static str {
      "pong!"
  }

  #[tokio::test]
  async fn it_sound_get() {
      // Build an application with a route.
      let app = Router::new()
          .route("/ping", get(get_ping))
          .into_make_service();

      // Run the server on a random address.
      let server = Server::new(app).unwrap();

      // Get the request.
      let response = server
          .get("/ping")
          .await;

      assert_eq!(response.contents, "pong!");
  }
```
