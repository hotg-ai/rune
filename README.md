# Rune

Rune is a containerization technology for deploying TinyML applications to extremely constraint devices. 


## Runefile 

A `Runefile` is similar to `Dockerfile` in that it is a text document that defines capabilities, processing blocks, feature transformation, models and model outputs to assemble the `Rune`. 

A simplistic example of this is would be:

*TODO: refine the below with an exact working version*

```
FROM runicos/base

CAPABILITY AUDIO audio

PROC_BLOCK runicos/fft fft

MODEL ./example.tflite model

RUN audio fft model

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
