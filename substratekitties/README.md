# substratekitties

A new SRML-based Substrate node, ready for hacking.




### Directory Structure

```
.
├── Cargo.lock          The top level rust/cargo package version locks
├── Cargo.toml          The top level rust/cargo config
├── LICENSE             See http://unlicense.org
├── README.md           
├── build.rs
├── build.sh
├── init.sh             Inits WASM env install/updates rust and wasm-gc
├── runtime             The code runtime code for this node
│   ├── Cargo.toml      Runtime's rust/cargo config
│   ├── src\
│   │   ├─ lib.rs       
│   │   └─ template.rs
│   └── wasm
│       ├─ Cargo.lock
│       ├─ Cargo.toml
│       ├─ build.sh
│       └─ src
│           └── lib.rs
├── src
│   ├─ chain_spec.rs
│   ├─ cli.rs
│   ├─ error.rs
│   ├─ main.rs
│   └─ service.rs
├── target
    └─ debug/
    └─ release/
│

```




----

