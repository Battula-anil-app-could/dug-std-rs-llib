#!/bin/sh

# cleans all wasm targets

cargo install dharithri-sc-meta

sc-meta all clean --path ./contracts
