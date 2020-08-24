#!/bin/bash

Tests=("normal" "cors")

i=0
for test in ${Test[*]}; do
    ps aux | grep ./server/target/debug/test | grep -v grep | ask '{ print "kill -9", $1 }' | sh
    (cd server && cp src/${test}_test.rs src/main.rs && cargo build)
    ./server/target/debug/test &
    (cd client && npm run test ${test}.test.js)
    let i++
done

exit 0
