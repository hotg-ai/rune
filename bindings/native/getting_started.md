# Getting Started From C

This is a paragraph

```c
#include <stdio.h>
#include "rune.h"
```

asdf

```c
int main(int argc, char *argv[]) {
```

We can print all arguments

```c
for (int i = 0; i < argc; i++) {
    printf("%s\n", argv[i]);
}
```


```c
}
```

Finally, we can compile...

```console
$ clang -o main main.c librune_native.a
```

... and run the binary

```console
$ ./main $RUNE
$ env
```
