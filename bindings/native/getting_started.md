# Getting Started With the Native Library

How you provide Rune with your input tensors will vary depending on the
application, but the typical flow for an interactive application is

1. Load the Rune
2. For each input, initialize the corresponding widget/sensor and set it up to
   provide data in the desired format
4. An event occurs (e.g. a button press or timer) - pass inputs to the Rune
   runtime and trigger inference
5. Display the output tensors
6. Go to step 2

## Program Setup

First, we'll need to include the `rune.h` header and some other things for
interacting with the OS.

```c
#include <math.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include "rune.h"
```

We'll also declare some helper functions.

```c
int read_file(const char *filename, uint8_t **buffer);
__attribute__((noreturn)) void handle_error(Error *error);
void log_func(void *user_data, const char *msg, int len);
void close_log_file(void *user_data);
```

Before using a Rune we will first need to get it into our program. This is where
our `read_file()` helper comes in.

```c
int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <rune>\n", argv[0]);
        return 1;
    }

    uint8_t *rune = NULL;
    int bytes_read = read_file(argv[1], &rune);

    if (bytes_read < 0) {
        perror("Unable to read the Rune");
    }
```

## Loading the Rune

We need to provide the Rune runtime with some information before it can start
loading the Rune. The most important of which is a pointer to our Rune in
memory, but there are also options for switching between WebAssembly virtual
machines and so on.

```c
struct Config cfg = {
    .rune = rune,
    .rune_len = bytes_read,
    .engine = Wasm3,
};
```

It's safe to leave everything else with its default zero value.

Now we can load the Rune.

```c
Runtime *runtime = NULL;
Error *error = rune_runtime_load(&cfg, &runtime);
```

It's possible that loading will fail, in which case we need to check the return
value and handle it accordingly.

We'll cover error handling in a bit, so for now, let's call [a helper
function](#error-handling) and continue.

```c
if (error) {
    handle_error(error);
}
```

The runtime also lets you pass in a callback triggered on each log message, as
well as a pointer to some `user_data` object and a function for cleaning up that
object afterwards.

In this case, we'll log all messages to a file.

```c
FILE *f = fopen("rune.log", "w");
if (!f) {
    perror("Unable to open the log file");
}

rune_runtime_set_logger(runtime, log_func, f, close_log_file);
```

The implementation of `log_func` just writes messages to `f`. Check out the
[*Logging*](#logging) section if you want to see how it works.

## Specifying Inputs

To make this example simpler, our application will generate dummy values instead
of connecting to any peripherals or widgets.

The starting point for inspecting a Rune's inputs is the `Metadata` object.

```c
Metadata *inputs;
error = rune_runtime_inputs(runtime, &inputs);
if (error) {
    handle_error(error);
}
```

This object contains a readonly snapshot of the input metadata, so it is safe
to hold onto long term (e.g. by saving it as a field in your main application
object).

We can iterate through the `Metadata` to find out more about each node.

First, you look up each node in the list.

```c
printf("Input Nodes:\n");

for(int i = 0; i < rune_metadata_node_count(inputs); i++) {
    const Node *node = rune_metadata_get_node(inputs, i);

```

All nodes have a `id` which uniquely describes them, plus a `kind` which tells
you what type of input it is (`RAND`, `IMAGE`, .etc.). You'll often use the
`kind` to figure out what widget to display for a particular input.

```c
// Each node is given a unique node ID and knows what kind of node it is
uint32_t id = rune_node_id(node);
const char *kind = rune_node_kind(node);

printf("%s [id: %d]\n", kind, id);
```

A Runefile's author may embed arguments into it. These arguments are used to
modify parameters associated with a particular input.

For example, if an `IMAGE` input asks for a `width` of `244` pixels, the host
application can select the camera resolution accordingly or use dedicated
hardware to rescale it.

```c
    for(int arg_no = 0; arg_no < rune_node_argument_count(node); arg_no++) {
        const char *name = rune_node_get_argument_name(node, arg_no);
        const char *value = rune_node_get_argument_value(node, arg_no);
        printf("\t%s = %s\n", name, value);
    }
}
```

Normally you would generate input tensors based on the requested inputs and
their arguments, but we can cut some corners because we know this example will
only be run on the Sine Rune.

In this case, the Sine Rune has a single `RAW` input, with an `id` of `1` and
returning a `f32[1, 1]` tensor.

Writing an input tensor requires first getting the `InputTensors` object (think
of it like a dictionary that maps input `id`s to their tensors).

```c
InputTensors *input_tensors = rune_runtime_input_tensors(runtime);
```

Next, we'll need to tell the `InputTensors` to create a new tensor. This
requires knowing the input's node ID (we know it's `3` because we are cheating),
plus the element type and dimensions.

```c
int raw_node_id = 1;
size_t dimensions[2] = { 1, 1 };
Tensor *raw_input = rune_input_tensors_insert(
    input_tensors,
    raw_node_id,
    F32,
    dimensions,
    2
);
```

This gave us back a pointer to a `Tensor` that we can write to.

```c
float value = 0.8; // Note: sin(0.8) = 0.6972786, according to this model
uint8_t *raw_input_buffer = rune_tensor_buffer(raw_input);
memcpy(raw_input_buffer, &value, sizeof(float));
```

Don't forget to free the `Metadata` and `InputTensors` objects once you are done
with them!

```c
rune_input_tensors_free(input_tensors);
rune_metadata_free(inputs);
```

It is particularly important to free the `InputTensors` before touching the
runtime again because it contains references to the runtime's internals that
may be invalidated.


## Inference

At some point later on, perhaps in response to a button press or a new frame
from the camera, we will want the Rune to predict a result.

```c
error = rune_runtime_predict(runtime);
if (error) {
    handle_error(error);
}
```

## Reading Outputs

Once `rune_runtime_predict()` has completed, the runtime's output tensors will
be populated.

There can be multiple tensors per output so we need to create an `OutputTensors`
object to iterate over them.

```c
OutputTensors *outputs = rune_runtime_output_tensors(runtime);
```

```c
const OutputTensor *output_tensor;
uint32_t output_id;

printf("Reading outputs:\n");

while(rune_output_tensors_next(outputs, &output_id, &output_tensor)) {
    const Tensor *fixed = rune_output_tensor_as_fixed(output_tensor);

    if (!fixed) {
        printf("Skipping %d (not a fixed size output tensor)\n", output_id);
        continue;
    }

    ElementType element_type = rune_tensor_element_type(fixed);
    size_t rank = rune_tensor_rank(fixed);
    const size_t *dimensions = rune_tensor_dimensions(fixed);

    if (element_type == F32 && rank == 2 && dimensions[0] == 1 && dimensions[1] == 1) {
        float value = *(float *)rune_tensor_buffer_readonly(fixed);
        printf("\tOutput %d = [[%f]]\n", output_id, value);
    }

}
```

It is important to free the `OutputTensors` before we can touch the `Runtime`
again.

```c
rune_output_tensors_free(outputs);
```

## Cleaning Up

Finally, we need to free all allocated memory and exit successfully.

```c
    rune_runtime_free(runtime);
    free(rune);

    return 0;
}
```

## Compiling

Compiling the program is fairly simple. You should be able to link with the
`librune_native.a` binary.

```console
$ clang -o main main.c librune.a -lm -Wall -Werror -Wpedantic -L$RUNECORAL_DIST_DIR/lib/linux/x86_64 -lrunecoral -lstdc++
```

... and then run it:

```console
$ ./main $RUNE
```

```text
Input Nodes:
RAW [id: 1]
	length = 4
Reading outputs:
	Output 3 = [[0.697279]]
```

Okay, technically `sin(0.8) = 0.7173561`, but this model isn't very accurate
and `0.697279` is exactly what it predicts in training, too.

While we are at it, we can also see the messages that were logged.

```console
$ cat rune.log
```

```text
{"level":"DEBUG","message":"Running the pipeline","target":"hotg_runicos_base_wasm::guards","module_path":"hotg_runicos_base_wasm::guards","file":"/home/consulting/Documents/hotg-ai/rune/images/runicos-base/wasm/src/guards.rs","line":66}
{"level":"DEBUG","message":"Reading data from \"rand\"","target":"sine","module_path":"sine","file":"lib.rs","line":70}
{"level":"DEBUG","message":"Executing \"mod360\"","target":"sine","module_path":"sine","file":"lib.rs","line":72}
{"level":"DEBUG","message":"Executing \"sine\"","target":"sine","module_path":"sine","file":"lib.rs","line":74}
{"level":"DEBUG","message":"Sending results to the \"serial\" output","target":"sine","module_path":"sine","file":"lib.rs","line":76}
{"level":"DEBUG","message":"Pipeline finished","target":"hotg_runicos_base_wasm::guards","module_path":"hotg_runicos_base_wasm::guards","file":"/home/consulting/Documents/hotg-ai/rune/images/runicos-base/wasm/src/guards.rs","line":80}
{"level":"DEBUG","message":"Pipeline Stats { allocations: 22, deallocations: 22, reallocations: 0, bytes_allocated: 665, bytes_deallocated: 665, bytes_reallocated: 0 }","target":"hotg_runicos_base_wasm::guards","module_path":"hotg_runicos_base_wasm::guards","file":"/home/consulting/Documents/hotg-ai/rune/images/runicos-base/wasm/src/guards.rs","line":22}
```

## Error Handling


```c
__attribute__((noreturn))
void handle_error(Error *error) {
    char *msg = rune_error_to_string_verbose(error);
    printf("%s\n", msg);
    free(msg);
    rune_error_free(error);
    exit(1);
}
```

## Logging

Our logger implementation just writes to a file, making sure to append a newline
and flush so users will immediately see when errors occur.

```c
void log_func(void *user_data, const char *msg, int len) {
    FILE *f = (FILE *)user_data;

    fwrite(msg, len, sizeof(char), f);
    fwrite("\n", 1, sizeof(char), f);
    fflush(f);
}

void close_log_file(void *user_data) {
    FILE *f = (FILE *)user_data;
    if (fclose(f)) {
        perror("Unable to close the log file");
    }
}
```

## Other Helper Functions

Our `read_file()` helper isn't particularly interesting. All it does is find out
how large a file is, allocate a suitably sized buffer, then read the file's
contents into that buffer.

```c
int read_file(const char *filename, uint8_t **buffer) {
    FILE *f = fopen(filename, "r");
    if (!f) {
        return -1;
    }

    fseek(f, 0, SEEK_END);
    int length = ftell(f);
    fseek(f, 0, SEEK_SET);

    *buffer = (uint8_t*)calloc(length, sizeof(uint8_t));
    return fread(*buffer, sizeof(uint8_t), length, f);
}
```
