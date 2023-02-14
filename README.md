# Word 2 Emotions

This is a little personal project to experiment a little bit with WASM, Rust and Sevelte.
The objective is to create a SPA that can use the WebSpeech API to recognize speech, then
convert the user voice and try to pedict what they are filling with NLP.

## Tech Stack

- SvelteKit
- TailwindCSS
- Rust

## Development

You have to make certain steps if you want to tun WASM in SvelteKit, unfortunlty I havent
made it run using the hot reload fucntion Svelte, only building and previewing :c
The steps you must follow to connect WASM are :

1. Make a Cargo lib with

`cargo new --lib cratename`

2. Add this to your Cargo.toml

```
[package]
name = "cratename"
version = "0.1.0"

# See more keys and their definitions at:
# https://doc.rust-lang.org/cargo/reference/manifest.html

# We have to define an edition explicitly, otherwise we can't compile.
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.83"

wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.33"

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

3. Compile using the fallowing flags :

`wasm-pack build --target web --release`

4. Rename the pkg folder to cratename and move it to /src

5. Add the following line in package.json

`"cratename": "file:src/cratename"`

6. Run

`npm i`

7. That's it! (If you have more questions send me a message)

## Deployment

I'm working in making an automated CI/CD with GitHub Actions!
