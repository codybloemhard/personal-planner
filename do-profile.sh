#!/bin/bash
cargo build --release
perf record --call-graph=dwarf ./target/release/pplanner
perf report --hierarchy -M intel
rm *.data
rm *.data.old