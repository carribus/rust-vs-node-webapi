# NodeJS vs Rust wep api perf comparison

This was a quick experiment to compare the complexity and performance associated with building a microservice in NodeJS vs Rust.

## What is in the web service

A simple fibonacci sequence calculator endpoint that accepts a URI parameter to define the length of the sequence that should be calculated. The returned result is a string containing the summed value. The algorithm used is the same in all implementations (which means it was not optimised for Rust) and provides a time complexity of `O(N)` and a constant space complexity.

## NodeJS & Express

For the NodeJS inmplementation, I used the current version of Express.js to provide the plumbing for the web api. As would be expected, pretty stock-standard stuff

## Rust & Actix vs Rust & Warp

For the Rust based implementation, I created an implementation using Actix-web as well as Warp (both are excellent crates!).

## Benchmarking

I used [`wrk`](https://github.com/wg/wrk) to benchmark the services.

Node Service listens on port 3000
Rust-actix service on port 3001
Rust-warp service on port 3002

The command I used for my benchmarks is:

```
$ wrk -t12 -c400 -d30s http://localhost:{port}/fib/50
```

Since NodeJS is already running in release build mode, it is only fair to build the Rust apis in release mode as well. To do this, simply `cargo run --release` in the workspace or in the individual crate folder.