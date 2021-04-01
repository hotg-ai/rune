# Tutorial

This will be a  walkthrough on creating a rune to predict which gesture is being made by a device (using the accelerometer).

## Installation & Dependencies

You will need to install rust on your device. You can follow their [offical documentation](https://doc.rust-lang.org/book/ch01-01-installation.html) to do so.

The next step is to have the required dependencies installed. Run the following commands in your terminal.

<<<<<<< HEAD
1. `rustup install nightly` if you already have nightly installed, make sure you are running the latest version with `rustup update nightly`
2. `rustup target add wasm32-unknown-unknown`
=======
1. `rustup install nightly`
2. `rustup default nightly`
3. `rustup target add wasm32-unknown-unknown`
4. `brew install binaryen`
>>>>>>> 0ce60b28593bfb4b5f24d8b4ac6b73729423d2ef

## Creating a Project

### Setup

The first step is to clone the [rune directory](https://github.com/hotg-ai/rune).

We will begin by creating a `Runefile` in the folder under `rune/docs/tutorial_gesture` with the following commands. Make sure you are in the cloned repo before running the commands.

1. `cd docs`
2. `cd tutorial_gesture`
3. `touch Runefile`

*Note: The Runefiles for other things like microspeech and person detection can be found in the `examples` directory*

### Runefile

The following is a basic template of a Runefile. Please read [What's In a Rune](https://tinyverse.substack.com/p/whats-in-a-rune) to understand the different Directives that the Runfile has before completing the next step.

```rust
FROM runicos/base

DIRECTIVE<{Input}> {label_1} {TYPE} {optional_parameter}

DIRECTIVE<{Input},{Output}> {label_2} {file_location}

OUT serial

RUN {label_1} {label_2} serial
```

There are 3 Directives that can be called. The data input and output needs to be specified as follows.

1. `CAPABILITY` (Input Only)
2. `MODEL` (Input and Output)
3. `PROC_BLOCK` (Input and Output)

Each of these has an `{Input}` and `{Output}` which follow the following format:
      `{Data Type}[array]` i.e. `f32[128, 3]`
The next step in the creation of the Runefile is to name each of the directives by changing `{label_#}`. The label of each stage will be used to execute them in the final `RUN` directive.

The `CAPABILITY` directive processes the initial data according to what is required. We currently support

- RAND (generates buffer with random data)
- SOUND (consumes sound)
- ACCEL (consumes x, y, z data from the accelerometer)
- IMAGE (consumes an image of x, y )
- IMAGE (consumes an image of  x, y)

The `PROC_BLOCK` directive allows for pre- and post-processing of the data. For example, if the model needs the data to be normalized to values between 0 and 1, you can create a `PROC_BLOCK` to perform this pre processing.

```rust
PROC_BLOCK<f32[128, 3],f32[128, 3]> normalize hotg-ai/rune#proc_blocks/normalize
```

The input and output of the Procedural Block are the same `f32[128, 3]`. The label for the Procedural block is `normalize` and the location of the Procedural Block is `hotg-ai/rune#proc_blocks/normalize`.

The `MODEL` directive tells the rune to use the tflite model.

The `OUT` directive sends the data to be consumed by the host  device. serial output sends the data as JSON.

Populate the Runefile with the following:

```rust
FROM runicos/base

CAPABILITY<f32[128, 3]> accelerometer ACCEL -n 128

PROC_BLOCK<f32[128, 3],f32[128, 3]> normalize hotg-ai/rune#proc_blocks/normalize

MODEL<f32[128, 3],f32[4]> gesture ./model.tflite

PROC_BLOCK<f32[64], UTF8> gesture_agg hotg-ai/rune#proc_blocks/gesture_agg --labels=Wing,Ring,Slope,Unknown

OUT SERIAL

RUN accelerometer normalize gesture gesture_agg serial
```

>CAPABILITY will process incoming data into a floating point `128 * 3` array.

>Procedural Blocks are used to do things to the data. The normalize PROC_BLOCK is used to compress the incoming data between 0 and 1.

>The data is then run through the tflite MODEL producing an output of 4 floating point numbers with different levels of confidence of the model (i.e. `[0.0, 1.0, 0.0, 0.0]`).

>The gesture_agg PROC_BLOCK takes the ouptut of the model, and returns a UTF8 from the labels provided (`--labels=Wing, Ring, Slope, Unknown`)
(example output based on previous model output: `Ring`).

>The UTF8 label is then sent to the serial which can be read by whichever device the rune is being deployed to.

### Procedural Blocks

The necessary PROC_BLOCKS to be used in `tutorial_gesture` have already been created and are stored in `proc_blocks/`

Further Tutorials shall explain how to create your own procedural block.

### Model

Trained tflite models should be placed in the same directory as the `Runefile` (because the `Runefile` is expecting the model to be located in `./model.tflite`). A different location can be used but the MODEL line in the `Runefile` will need to be updated to point to the changed location.

For convenience `docs/tutorial_gesture` already contains the tflite model we will be using for this tutorial.

### Build

Great! Now that everything is setup, we can generate the rune :)

1. Open terminal and go to the `rune` directory. Run the next command.
2. `cargo run --bin rune -- build docs/tutorial_gesture/Runefile`

Your rune `tutorial_gesture.rune` will be generated in the same directory as the `Runefile`.

Congrats! You now have a rune which can be deployed on your devices.

### Testing

You can run the Rune with test data which has been provided in the `docs/tutorial_gesture` directory using the following command.

>```cargo run --bin rune -- run docs/tutorial_gesture/tutorial_gesture.rune --capability accelerometer:docs/tutorial_gesture/example_ring.csv```

`example_ring.csv` contains data similar to what an accelerometer would collect if a ring gesture was made.

The Serial output of running the above command should be "Ring"