# Implementation of a Processing Block

## Intro

In our first tutorial, we created a Runefile which contains two `PROC_BLOCK` directives.

The **first** directive

```rust
PROC_BLOCK<f32[128, 3],f32[128, 3]> normalize hotg-ai/rune#proc-blocks/normalize
```

is used to pre-process the data by calling a proc block to normalize the data to values between 0 and 1.

The **second** directive

```rust
PROC_BLOCK<f32[64], utf8> gesture_agg hotg-ai/rune#proc-blocks/gesture_agg --labels=Wing,Ring,Slope,Unknown
```

is used to post-process data. The tflite model, which we are using to predict which gesture is being made, outputs data as a 32 bit floating point array which contains 4 values.

We want the processing block to take in an aggregate of the model ouptut which generates a list of confidences, and return the most confident gesture.

The input of the gesture_agg directive is taken as `f32[64]`, and its output shall be `utf8` since we want it to return one of the labels provided `Wing,Ring,Slope,Unknown`.

## Setup

Let's begin by generating the gesture_agg processing block files. Run the following Cargo command in the `proc-blocks` folder.

```bash
cargo new gesture_agg --lib
```

The `runic_types` library needs to be added to the generated Cargo.toml file under dependencies.

```rust
[dependencies]
runic-types = { path = "../../runic-types" }
```

## Writing a Processing Block

Now that the setup is complete, we can focus our attention to the lib.rs file where the bulk of our code is going to be written.

Let's begin by using the template from the previous tutorial to populate our lib.rs file.

```rust
#![no_std]

use runic_types::Transform;

pub struct GestureAgg<const N: usize> {
    genericParameter: N,
}

impl<const N: usize> Transform<[f32; N]> for GestureAgg<N>{
    type Output = &'static str;
    fn transform(&mut self, input: [f32; N]) -> Self::Output {
        // Logic goes here
    }
}
```

You may have noticed that we have tweaked a few things.

- The name of the processing block has been replaced, and we have replaced `T` with `const N: usize` to denote the number of elements in the array input in the Runefile.
- Data type of the output (which is a string literal) is `&'static str`
- The transform function is taking in `[f32; N]` input.

We will be taking advantage of a wonderfull feature that Rust has... generics! With this, the processing block will work even if the input array size in the Runefile has been changed.

It will become extremely messy if we were to write the logic behind choosing the most confident gesture in a single method. So let's create another one and place it between the struct and the Transform method.

We will need a few different functions to find the correct gesture. We'll need an `add_history` function to add and update the history of confidences. A `most_likely_gesture` function to identify the most probably gesture. A `label_for_index` function to be able to call the index of the most likely gesture. `with_throttle_interval` function to prevent an excessive number of gesture being added to the history in quick succession. A `with_labels` function to match the confidence values with the labels provided in the Runefile. Finally, we'll also need a `new` function where we can add all the common properties of the other functions.

```rust
pub struct GestureAgg<const N: usize> {
    ...
}

impl<const N: usize> GestureAgg<N> {
    pub fn new() -> Self {
        GestureAgg {
            labels: [""; N],
            history: VecDeque::new(),
            max_capacity: MAX_CAPACITY,
            unknown: UNKNOWN_LABEL,
            throttle_interval: DEFAULT_THROTTLE_INTERVAL,
            countdown: 0,
        }
    }

    pub fn with_labels(self, labels: [&'static str; N]) -> Self {
        GestureAgg { labels, ..self }
    }

    pub fn with_throttle_interval(self, throttle_interval: usize) -> Self {
        GestureAgg {
            throttle_interval,
            ..self
        }
    }

    fn add_history(&mut self, input: [f32; N]) {
        self.history.push_back(input);

        while self.history.len() > self.max_capacity {
            self.history.pop_front();
        }
    }

    fn most_likely_gesture(&self) -> Option<usize> {
        if self.history.is_empty() {
            return None;
        }

        (0..N)
            .fold(None, |previous_most_likely, gesture_index| {
                let sum: f32 =
                    self.history.iter().map(|input| input[gesture_index]).sum();
                let avg = sum / self.history.len() as f32;

                match previous_most_likely {
                    Some((_, previous_avg)) if previous_avg >= avg => {
                        previous_most_likely
                    },
                    _ => Some((gesture_index, avg)),
                }
            })
            .map(|pair| pair.0)
    }

    fn label_for_index(&self, index: Option<usize>) -> Option<&'static str> {
        index.and_then(|ix| self.labels.get(ix)).copied()
    }
}
```

In the `new` function, `max_capacity`, `unknown`, and `throttle_interval` were assigned to some constants. Let's add them in above the newly created implementation.

```rust
const MAX_CAPACITY: usize = 1024;
const UNKNOWN_LABEL: &'static str = "<MISSING>";
const DEFAULT_THROTTLE_INTERVAL: usize = 16;
```

The `GestureAgg` structure needs to be defined.

```rust
pub struct GestureAgg<const N: usize> {
    genericParameter: N,
}
```

The generic parameter needs to be replaced by the definitions of the parameters which are in the `new` function.

```rust
pub struct GestureAgg<const N: usize> {
    labels: [&'static str; N],
    history: VecDeque<[f32; N]>,
    max_capacity: usize,
    unknown: &'static str,
    throttle_interval: usize,
    countdown: usize,
}
```

Notice that since we are using VecDeque in the processing block, we need to declare the VecDeque module. Add the `use` declaration below the `runic_types` declaration.

```rust
#![no_std]

use runic_types::Transform;
use alloc::collections::VecDeque;
```

Now that all the functions work, we need to put them together in the Transform method. Add the following to the `transform` function in the `Transform` method.

```rust
    fn transform(&mut self, input: [f32; N]) -> Self::Output {

        self.add_history(input);
        let gesture_index = self.most_likely_gesture();
        let label = self.label_for_index(gesture_index);
        self.countdown = self.countdown.saturating_sub(1);

        match label {
            Some(label) if self.countdown == 0 => {
                self.countdown = self.throttle_interval;
                label
            },
            _ => self.unknown,
        }
    }
```

There is just 1 more step needed for the proc block to work. We need to add a `Default` method with a `default` function. This can be added to the end of the processing block outside the `Transform` method.

```rust
impl<const N: usize> Default for GestureAgg<N> {
    fn default() -> Self { GestureAgg::new() }
}
```

Congratulations! You have succesfully created a processing block which can identify the most confident gesture from a list of confidences!
