#!/bin/bash

mkdir -p generated/ffi

bindgen \
    --no-copy '.*' \
    ./simplex-chat/packages/simplex-chat-nodejs/cpp/simplex.h \
    -o ./generated/ffi/bindings.rs \
    -- -x c++
