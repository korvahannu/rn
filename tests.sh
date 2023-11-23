#!/bin/bash

rm ./rn-tests-data/*
cargo test
rm -r ./rn-tests-data