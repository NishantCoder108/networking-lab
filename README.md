# networking-lab

A Rust experiment that builds HTTP out of a raw TCP byte stream, using only `std::net`. No HTTP crate. Each binary is one step up the stack, so the bytes a browser and `curl` already assume stay visible.

## Direction

1. **TCP is an ordered byte stream** (`server` + `client`, port 9000). The client opens a socket, writes one line, and reads the reply. The server accepts one connection at a time, reads whatever arrived in a single `read` (at most 1024 bytes), prints it, and writes a fixed reply. The exchange ends because each side drops the socket after that one read and one write. Nothing in this pair frames a message: 1024 bytes is a buffer size, and the connection close is the boundary.

2. **HTTP is text on that same stream** (`networking-lab`, port 8080). The accept loop is the same. The request bytes are read into a buffer and then discarded, so every path gets the same `200`. The response is written by hand as HTTP/1.1: a status line, headers separated by `\r\n`, a blank line, then the body. `Content-Length` is `body.len()` in bytes, including the trailing newline (44 for the current body). `Connection: close` matches what the server does: it writes once and drops the socket, and the client treats that close as the end of the response.

The code is set up to answer these next, in order:

1. Parse the request line and headers out of the buffer the HTTP server already reads, and branch the body on method and path.
2. Treat `read` as a partial fill. A request can arrive in more than one `read`, and a short read is a prefix of the message.
3. Keep the socket open and use `Content-Length` as the frame, instead of closing after one response.
4. Serve the next client while one connection is still in progress. `incoming()` is blocking and serial today; one slow `read` holds the loop. A `read` error also uses `?` inside that loop, so it stops the process.

## Binaries

| Binary | Source | Address | Role |
| --- | --- | --- | --- |
| `networking-lab` | `src/main.rs` | `127.0.0.1:8080` | Hand-written HTTP/1.1 response |
| `server` | `src/bin/server.rs` | `127.0.0.1:9000` | TCP server |
| `client` | `src/bin/client.rs` | connects to `:9000` | TCP client |

`Cargo.toml` has no dependencies (edition 2021) and no `default-run`. `cargo run` with no `--bin` refuses to choose among the three binaries.

Both servers bind `127.0.0.1` only.

## Run

Start `server` before `client`. Each long-running binary needs its own terminal.

```sh
cargo run --bin networking-lab   # HTTP server on :8080
cargo run --bin server           # TCP server on :9000
cargo run --bin client           # one exchange with :9000
```

HTTP, with the wire trace:

```sh
curl -v http://127.0.0.1:8080/
```

`curl -v` is the way to see the status line, headers, blank line, and body. The request text belongs on the socket; typing `GET / HTTP/1.1` in the shell runs it as a command.

TCP, typed by hand:

```sh
nc 127.0.0.1 9000
```

## One exchange

TCP, port 9000. The client writes `Hello Server, can you hear me?\n`. The server writes `Hello from Rust TCP Server!\n`.

HTTP, port 8080. The response on the wire:

```http
HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Content-Length: 44
Connection: close

Hello from our custom raw Rust HTTP server!
```
