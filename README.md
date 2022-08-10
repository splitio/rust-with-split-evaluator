# Rust with the Split Evaluator

Simple example of using the Split Evaluator with the Rust programming language.  Shows using an HTTP cliient to GET a feature flag evaluation.

You must have rust and cargo installed (e.g. by using brew).

 - Clone the repository

```
git clone https://github.com/splitio/rust-with-split-evaluator.git
```

 - Build the example

```
cd rust-with-split-evaluator/
cargo build 
```

A lot of downloading and compiling ensues.

Once finished, you can...

```
cargo run
```

Output should look something like this...

```
     Running `target/debug/http`
"{\"new_onboarding\":{\"treatment\":\"off\",\"config\":null},\"multivariant_demo\":{\"treatment\":\"red\",\"config\":\"{\\\"text\\\":\\\"Adopt a  Dog\\\",\\\"image\\\":\\\"http://www.cortazar-split.com/dog_origin.jpeg\\\",\\\"image_width\\\":\\\"100%\\\"}\"}}"
```

There are a lot of third party dependencies that you can find documented in the Cargo.toml

Author:
david.martin@split.io

