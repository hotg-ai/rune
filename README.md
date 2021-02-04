# Rune

Rune is a containerization technology for deploying TinyML applications to extremely constraint devices. 

# Deps

1. rustup install nightly 
2. brew install binaryen
2. rustup target add wasm32-unknown-unknown


## Runefile 

A `Runefile` is similar to `Dockerfile` in that it is a text document that defines capabilities, processing blocks, feature transformation, models and model outputs to assemble the `Rune`. 

A simplistic example of this is would be:

*TODO: refine the below with an exact working version*

```
FROM runicos/base

CAPABILITY AUDIO audio --hz 16000 --samples 150 --sample-size 1500 

PROC_BLOCK runicos/fft fft

MODEL ./example.tflite model --input [150,1] --output 1

RUN audio fft model 

OUT serial
```

In this example a audio with fft (fast fourier transformation) block can be run with the model. 

### Usage

#### Build

Using the `rune` cli you can build containers 
that are tagged and available.

*List available containers*

`rune container ls`

*Build new containers*

`rune build .`

*Run the containers locally simulated*

`rune exec ${CONTAINER-ID}`


## Building and Running this project

- Install Rust from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)
- Build the project with `cargo build`
- This should create Rune executable in `./target/debug/rune`
- Run the project with `cargo run`


## Private Git Repos

To get deps from our private git repos we need to
use `ssh agent`. 

Add the below to your `.ssh/config` 
```
Host github.com
   UseKeychain yes
   AddKeysToAgent yes
   IdentityFile ~/.ssh/id_rsa
```

and run:
`ssh-add -K ~/.ssh/id_rsa` 