#!bin/sh

gcc -fPIC -shared   \
    -o libfoo.so    \
    foo.c

rustc -l foo        \
    -L .            \
    main.rs

env LD_LIBBARY_PATH="$LD_LIBBARY_PATH:." ./main