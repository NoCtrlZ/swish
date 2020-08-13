# Note
## Test
- Unit test should be written in each module file.
- Integration test should be written in `src/lib.rs`.
## Todo
- Capacitate user to set contents length limitation.
- Support almost status code.
- Write unit test and integration test separately.
- Box trait dyn should be replaced with enum.
- Add method field to router to optimize algorithm.
- Return content length.
- Header should be hashmap.
- Add config struct to response to compile header.
## Feature
- DBMS client.
- Mailer client.
## Refactoring
- Change test module structure.
- Change directory structure `*.rs -> */mod.rs`.
## Bugs
- Can't handle root routing.
- Can't handle not found and other error status.
## Point
- If it's necessary to import other module for test, it should be written as integration test.
- Public function should be tested in lib.rs and private function should be tested in file it located.
