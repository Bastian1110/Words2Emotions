# Word 2 Emotions

![Hero](hero.png 'WASM + SveltKit + AI')

This is a personal project to experiment a little bit with WASM, Rust and Svelte.
The objective is to create a SPA that can use the WebSpeech API to recognize speech, then
convert the user voice and try to pedict what they are filling with [NLP](https://en.wikipedia.org/wiki/Natural_language_processing).

## Tech Stack

Please take a look to [linfa](https://github.com/rust-ml/linfa), Machine Learning framework for Rust!

- SvelteKit
- Rust
- TailwindCSS

## AI NLP Model

![Explication](Simple.png 'Simple Explanation')

The idea is simple, vectorize a large set of sentences that correspond to an emotion, then train a [logistic regression](https://en.wikipedia.org/wiki/Logistic_regression) model based on the vectorized sentences where the targets are the emotions. Unfortunately it has not been as easy as I thought to implement this, since my only experience in machine learning has been with SK-learn and I had never used Rust in my life.

I know emotion dont work like this, they're more like a spectrum, you can't fully detect the emotions of something just from what it says, there are many factors at play, such as volume of voice and body language. This project is more to learn more about[NLP](https://en.wikipedia.org/wiki/Natural_language_processing).

## How to run WASM with SvelteKit

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

edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.83"

wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.33"

[profile.release]
opt-level = "s"
```

3. Compile using `wasm-pack`, with the fallowing flags :

`wasm-pack build --target web --release`

4. Rename the pkg folder to cratename and move it to /src

5. Add the following line in package.json

`"cratename": "file:src/cratename"`

6. Run

`npm i`

7. That's it! (If you have more questions send me a message)

## Deployment

This page its actually deployed using the Vercel hosting service (excelent DX, by the way), but the project is configured to build an SPA, not an SSR page, to test how the model work without having to relay a lot in the server.
