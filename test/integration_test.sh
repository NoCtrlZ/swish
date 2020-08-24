#!/bin/sh

./server/target/debug/test &
(cd client && npm run test normal.test.js)
ps aux | grep ./server/target/debug/test | grep -v grep | ask '{ print "kill -9", $1 }' | sh
(cd server && cp src/cors_test.rs src/main.rs && cargo build)
./server/target/debug/test &
(cd client && npm run test cors.test.js)
# exit 0
