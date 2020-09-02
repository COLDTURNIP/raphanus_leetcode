#!/bin/bash
runtype=$1
problem=$2
cargo test "${problem}::tests" --lib -- --nocapture "--$runtype"
