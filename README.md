# Substrate Kitties Workshop

From: https://shawntabrizi.github.io/substrate-collectables-workshop

```
$ mkdir substrate-kitties ; cd substrate-kitties/
$ get clone https://github.com/adnantium/substrate-kitties.git
$ mkdir substratekitties_chain
```
Should look something like:
```
$ tree -L 2
substrate-kitties/
    ├── LICENSE
    ├── README.md
    ├── substratekitties/
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   ├── ....
    │   ├── README.md
    │   ├── build.sh
    │   ├── runtime
    │   └── src/
    ├── substratekitties-ui
    │   ├── README.md
    │   ├── docs/
    │   ├── package.json
    │   ├── server.js
    │   ├── src/
    │   ├── webpack.config.js
    │   └── yarn.lock
    └── substratekitties_chain/
        └── ..../
```


```
$ cd substrate-kitties/substratekitties/
$ build.sh
$ cargo build --release
$ ./target/release/substratekitties -d ./substratekitties_chain --dev
```


