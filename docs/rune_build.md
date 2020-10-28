## Steps to generate rune container

### Update dependencies
```
cargo update
```

### Build rune binary

```
cargo build
```

### Build a rune container

```
./target/debug/rune build examples/microspeech/Runefile
```

this return the path to the rune container:
Generating rune container in "/Users/johndoe/.rune/runes/fd60c490-53df-4378-8f92-04917b74ee6a"

### Run a rune container
in this path, run rune container
```
cargo test -- --nocapture
```
(need to replace this by rune run)
