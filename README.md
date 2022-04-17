# HTTP Key Value Store

This is a simple, in-memory key-value store with an HTTP interface.

## HTTP API

There are two routes, one to set key-value pairs in the store and one to retrieve the value from a key

### Set

Send a POST request to `http://localhost:8000/set/:key` where `:key` is the key and the body is the string value (Content-type: text/plain)

### Get

Send a GET request to `http://localhost:8000/get/:key` where `:key` is the key

## Internals

The key-value store is a simple in-memory Rust `HashMap` that stores a String key and a byte array value. The `HashMap` is in a shared state, guarded by a Mutex, and wrapped in an atomic ref counter to ensure thread-safety while sharing the state across many connections concurrently.

Uses `tokio` and `warp` for async and HTTP request handling.

## Interacting

There are two helper scripts, `scripts/set.sh` and `scripts/get.sh` which make simple `curl` calls to the HTTP routes listed above, but you can use any HTTP client to make the calls.
