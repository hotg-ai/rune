# Procedural Blocks

In our first tutorial, we created a Runefile which contains two `PROC_BLOCK` directives.

The **first** directive

```rust
PROC_BLOCK<f32[128, 3],f32[128, 3]> normalize hotg-ai/rune#proc_blocks/normalize
```

is used to pre-process the data by calling a proc block to normalize the data to values between 0 and 1.

The **second** directive

```rust
PROC_BLOCK<f32[64], utf8> gesture_agg hotg-ai/rune#proc_blocks/gesture_agg --labels=Wing,Ring,Slope,Unknown
```

is used to post-process data. The tflite model, which we are using to predict which gesture is being made, outputs data as a 32 bit floating point array which contains 4 values. We want the procedural block to take in an aggregate of the model ouptut and make a prediction on which gesture is occurring.

The input of the gesture_agg directive is taken as `f32[64]`, and its output shall be `utf8` since we want it to return one of the labels provided `Wing,Ring,Slope,Unknown`.
