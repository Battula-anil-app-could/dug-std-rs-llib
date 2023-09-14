#!/bin/bash

cargo install dharithri-sc-meta

TARGET_DIR=$PWD/target

sc-meta all update --path ./contracts
