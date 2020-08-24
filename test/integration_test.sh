#!/bin/sh

./server/target/debug/test
(cd client && npm run test)
