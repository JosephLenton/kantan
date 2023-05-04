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

 * Comes with batteries included. No need to setup Hyper + Serde + Bytes + etc (again).
 * Can automatically save cookies and such from responses -- useful for logging in, and then making a followup request.
 * Can be setup to use headers, query urls, cookies, across multiple requests ahead of time.

**This is still an early work in progress.**
