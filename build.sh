#!/bin/bash
wasm-pack build --target web &&
ls -l ./static | awk 'match($0, /[A-z]*\.sass/) {
        in_name = "./static/" substr($0, RSTART, RLENGTH);
        out_name = "./static/" substr($0, RSTART, RLENGTH - 4) "css";

        system("sass " in_name " " out_name)
    }
' &&
rollup ./main.js --format iife --file ./pkg/bundle.js
