# Getting Started With the Native Library

The Rune runtime is available as a native library.

## Program Setup

First, we'll need to include the `rune.h` header and some other things for
interacting with the OS.

```c
#include <stdio.h>
#include <stdint.h>
#include "rune.h"
```

We'll also declare some helper functions.

```c
int read_file(const char *filename, uint8_t **buffer);
__attribute__((noreturn)) void handle_error(Error *error);
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

After loading the runtime you will need to find out which inputs have been
requested and any arguments they require.

```c
Metadata *inputs;
error = rune_runtime_inputs(runtime, &inputs);
if (error) {
    handle_error(error);
}
```

We can iterate through each node in the `Metadata` to find out more about them.

```c
printf("Input Nodes:\n");

for(int i = 0; i < rune_metadata_node_count(inputs); i++) {
    Node *node = rune_metadata_get_node(inputs, i);

    // Each node is given a unique node ID and knows what kind of node it is
    uint32_t id = rune_node_id(node);
    const char *kind = rune_node_kind(node);

    printf("%s [id: %d]\n", kind, id);

    // Nodes may also have arguments (e.g. an "IMAGE" node may have a "width"
    // argument)
    for(int arg_no = 0; arg_no < rune_node_argument_count(node); arg_no++) {
        const char *name = rune_node_get_argument_name(node, arg_no);
        const char *value = rune_node_get_argument_value(node, arg_no);
        printf("\t%s = %s\n", name, value);
    }
}
```

Don't forget to free the `Metadata` once you are done with it!

```c
rune_metadata_free(inputs);
```

If the Rune runtime is being embedded in a GUI application, this is a good point
to start creating widgets for each input node. For example, a `RAW` input may be
bound to a file picker, or you might activate the user's camera for an `IMAGE`
input using the `width` and `height` arguments to determine the resolution to
request.

## Inference

At some point later on, perhaps in response to a button press or a new frame
from the camera, we will want to use the Rune.

TODO:
- Set inputs
- Call the predict method
- Read the results

Finally, we need to tell the user that everything ran successfully.

```c
    free(rune);
    return 0;
}
```

## Compiling

Compiling the program is fairly simple. You should be able to link with the
`librune_native.a` binary.

```console
$ clang -o main main.c librune.a -lm -Wall -Wpedantic
```

... and then run it

```console
$ ./main $RUNE
```

The output will look something like this

```text
TODO: add output
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

    *buffer = calloc(length, sizeof(uint8_t));
    return fread(*buffer, sizeof(uint8_t), length, f);
}
```
