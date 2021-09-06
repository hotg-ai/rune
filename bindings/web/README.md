
# Getting started with web bindings



## Compiling web bindings with browserify

Add this package to your project directly or compile it to a standalone module

### Make sure you have the proper dependencies installed

```console
yarn global add browserify
yarn global add typescript
yarn install
```

### Create js module
```console

tsc && browserify -e dist/index.js -o examples/module/runevm.js -s rune
```
### Run the example app

Annoyingly, we need to copy some tflite scripts to the example app from the @tfjs/tflite node_modules directory:

```console
cp node_modules/@tensorflow/tfjs/dist/tflite_web_api_* examples/module/
```

Use a simple http server to test:
```console
cd example/module
python -m SimpleHTTPServer
```