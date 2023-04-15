<div align="center">
  <h1>
    Kantan<br>
    a simple way to make requests to a server
  </h1>

  [![crate](https://img.shields.io/crates/v/kantan.svg)](https://crates.io/crates/kantan)
  [![docs](https://docs.rs/kantan/badge.svg)](https://docs.rs/kantan)
</div>

Kantan is a simple way to make requests to a server.
With serde, ways to parse headers and cookies, and more, all built in.

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

### Runs on a random port, allowing multiple to run at once

When you start the server, you can spin it up on a random port.
Allowing you to run multiple servers in parallel.

This is to allow multiple E2E tests to run in parallel.
Each with their own webserver.

### Remembers cookies across requests

It is common in E2E tests that step 1 is to login, and step 2 is the main request.
To make this easier cookies returned from the server will be preserved,
and then included into the next request. Like a web browser.

### Fails fast on unexpected requests

By default; all requests will panic if the server fails to return a 200.
This can be switched to panic when the server _doesn't_ return a 200.

This is a very opinionated design choice, and is done to help test writers fail fast when writing tests.
