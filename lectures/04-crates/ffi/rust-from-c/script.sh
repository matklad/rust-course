#!bin/sh

rustc foo.rs --crate-type cdylib

gcc -l foo  \
    -L .    \
    -o main \
    main.c

env LD_LIBRARY_PATH="$LD_LIBRARY_PATH:." ./main

