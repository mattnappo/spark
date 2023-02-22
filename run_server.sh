#!/bin/bash

RUST_LOG=info cargo run --bin server -- 9090 ./data/db1/ ./data/336d78316b4c.esk
