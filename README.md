# Qho

A small programming language built in Rust, with a syntax inspired by the Ndebele language. This project was built as an experiment using [Kiro](https://kiro.dev) — pushing the limits of what an AI-assisted IDE can help you build from scratch in a single session.

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)

### Build

```bash
cargo build --release
```

The binary will be at `target/release/qho.exe` (Windows) or `target/release/qho` (Linux/macOS).

### Run a file

```bash
./target/release/qho myprogram.ndebele
```

Files must have the `.ndebele` extension.

---

## The Language

### Keywords

| Keyword | Meaning |
|---|---|
| `bhala` | print to console |
| `inombolo` | number type (float64) |
| `ibala` | string type |
| `yenza` | generic value binding (sockets, listeners, etc.) |
| `uma` | if |
| `phindaUma` | while loop |
| `phinda` | infinite loop (like Rust's `loop`) |
| `phuma` | break |
| `qhubeka` | continue |
| `isigoqelo` | function |
| `isakhi` | struct |
| `bumba` | instantiate a struct |
| `mina` | self (inside methods) |
| `phendukisa` | return |
| `akulalutho` | null value |

### Variables

```js
inombolo iminyaka = 25;
ibala ibizo = "Tawanda";
yenza conn = tcpXhumana("127.0.0.1:8080");
```

### Functions

```js
isigoqelo bingelela(name: ibala) {
    bhala("Sawubona,");
    bhala(name);
}

bingelela("Tawanda");
```

### Structs and Methods

```js
isakhi Server {
    ibala IP
    inombolo port

    isigoqelo qala() {
        bhala("Starting on:");
        bhala(mina.IP);
    }
}

bumba Server s;
s.IP = "127.0.0.1";
s.port = 8080;
s.qala();
```

### Control Flow

```js
uma (x > 5) {
    bhala("greater than 5");
}

phindaUma (x < 10) {
    bhala(x);
    inombolo x = 10;
}

phinda {
    bhala("forever");
    phuma;
}
```

### Comparison Operators

`==`, `!=`, `<`, `>`, `<=`, `>=`

---

## Networking (TCP)

All networking functions are built-in — no imports needed.

| Function | Description |
|---|---|
| `tcpLalela(addr)` | Bind and listen on an address |
| `tcpAmukela(listener)` | Accept an incoming connection |
| `tcpXhumana(addr)` | Connect to a remote server |
| `tcpFunda(stream)` | Read a line from a stream (returns `nil` on disconnect) |
| `tcpThumela(stream, msg)` | Send a message over a stream |
| `tcpVala(stream)` | Close a connection |

### Echo Server Example

```js
yenza listener = tcpLalela("127.0.0.1:8080");

phinda {
    yenza conn = tcpAmukela(listener);

    phinda {
        yenza msg = tcpFunda(conn);

        uma (msg == akulalutho) {
            tcpVala(conn);
            phuma;
        }

        tcpThumela(conn, msg);
    }
}
```

Run the server:
```bash
./qho examples/echo_server.ndebele
```

Run the client in a second terminal:
```bash
./qho examples/echo_client.ndebele
```

---

## Project Structure

```
src/
  main.rs          -- entry point, file loading
  lexer.rs         -- tokenizer
  parser.rs        -- AST builder
  interpreter.rs   -- tree-walk interpreter
  stdlib/
    mod.rs         -- stdlib module registry
    net.rs         -- TCP networking built-ins
examples/
  hello.ndebele
  server.ndebele
  echo_server.ndebele
  echo_client.ndebele
```

---

## Roadmap

### Done
- [x] Lexer and parser
- [x] Variables (`inombolo`, `ibala`, `yenza`)
- [x] Functions (`isigoqelo`)
- [x] Structs and methods (`isakhi`, `mina`)
- [x] Control flow (`uma`, `phindaUma`, `phinda`, `phuma`, `qhubeka`)
- [x] TCP built-ins (no imports)
- [x] Echo server

### Todo
- [ ] Implement a working TCP server (handle multiple clients)
- [ ] Implement WebSockets
- [ ] Write a small chat application in Qho
- [ ] HTTP server — serve a basic response over TCP
- [ ] Port scanner — scan a range of ports on a host
- [ ] Ping utility — check if a host is reachable
- [ ] Simple load balancer — round-robin between backend addresses
- [ ] DNS lookup built-in — resolve a hostname to an IP
- [ ] Arithmetic expressions (`+`, `-`, `*`, `/`)
- [ ] Variable reassignment
- [ ] Arrays / lists
- [ ] Standard input (`funda` from stdin)
- [ ] File I/O built-ins
- [ ] Error handling (`zama` / `phephisa` — try/catch)
- [ ] Concurrency — handle multiple connections at once

---

## Contributing

Contributions are welcome. The codebase is intentionally small and readable.

### How to contribute

1. Fork the repo
2. Create a branch: `git checkout -b my-feature`
3. Make your changes
4. Run the examples to make sure nothing is broken:
   ```bash
   cargo run -- examples/hello.ndebele
   cargo run -- examples/echo_server.ndebele
   ```
5. Open a pull request

### Where to add things

- New keywords → `src/lexer.rs` (add token) + `src/parser.rs` (add AST node + parse branch) + `src/interpreter.rs` (handle in `exec_stmt`)
- New built-in functions → `src/stdlib/net.rs` (or a new file like `src/stdlib/fs.rs`), then register in `interpreter.rs` inside `register_builtins()`
- New examples → `examples/*.ndebele`

### Code style

- Keep it simple — this is an interpreter, not a compiler
- Prefer clear error messages over panics
- Every new built-in should have a matching `.ndebele` example

---

## License

MIT
