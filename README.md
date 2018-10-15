# Wordlist
Parity Brain Wallets wordlist library


[Rust Documentation](https://docs.rs/parity-wordlist/)


# RUST

```toml
# Cargo.toml

[dependencies]
parity-wordlist = "1.2"
```

```rust
# main.rs

extern crate parity_wordlist;

fn main() {
  println!("Words: {}", parity_wordlist::random_phrase(12));

  let phrase = "violin oblivion cylinder list disarray wobbly fastball showplace oasis patronize septic spearhead";
  println!("Valid: {:?}", parity_wordlist::validate_phrase(phrase, 12));
}
```


# JavaScript


```bash
$ npm i @parity/wordlist --save
```


```js
// main.js

import { randomPhrase, verifyPhrase } from '@parity/wordlist'

console.log(randomPhrase(12))

// This will throw if the phrase is not valid:
verifyPhrase("violin oblivion cylinder list disarray wobbly fastball showplace oasis patronize septic spearhead", 12)
```

# browser wasm

Note that if using as a Nodejs module, 'Crypto' will be missing.

Using [wasm-pack](https://github.com/rustwasm/wasm-pack)
```bash
$ cargo build --release --target wasm32-release-release
$ #wasm-bindgen ./target/wasm32-unknown-unknown/release/parity_wordlist.wasm --out-dir pkg
$ wasm-pack build --target browser --scope parity
```
Target brawser eg with webpack
```
$ cd pkg
$ npm install webpack webpack-cli --save-dev
$ npx webpack
```
Sample index.js in pkg should display word in browser console (need to serve dist from a test server with support for wasm mime-type).

