
# Compiling and testing

## Compiling web bindings with browserify

Add this package to your project directly or compile it to a standalone module

### Make sure you have the proper dependencies installed

```
yarn global add browserify
yarn global add typescript
```

### Create js module
```
tsc && browserify -e dist/index.js -o examples/module/runevm.js -s rune
```
### Run the example app

Use a simple http server to test:
```
cd example/module
python -m SimpleHTTPServer
```