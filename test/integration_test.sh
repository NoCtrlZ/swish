#!/bin/sh

set -- "normal" "cors"

while [ $# -gt 0 ]
do
    ps aux | grep ./server/target/debug/test | grep -v grep | awk '{ print "kill -9", $1 }' | sh
    (cd server && cp src/$1_test.rs src/main.rs && cargo build)
    ./server/target/debug/test &
    (cd client && npm run test $1.test.js)
    shift
done

exit 0
