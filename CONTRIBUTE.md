# Note
## Versioning
`0.0.0`
- First: Changing without having compatibility.
- Second: Adding function with having compatibility.
- Third: Fixing bug with having compatibility.
## Todo
- [ ] Capacitate user to set contents length limitation.
- [ ] Support almost status code.
- [ ] Box trait dyn should be replaced with enum.
- [ ] Return date with header.
- [ ] Change name according to clean code.
- [ ] Error handling all function.
- [ ] Lifetime optimization, remove clone and copy.
- [ ] Support query params.
## Feature
- [ ] DBMS client.
- [ ] Mailer client.
## Refactoring
- [ ] Change directory structure `*.rs -> */mod.rs`.
## Bugs
- [ ] Can't handle root routing.
- [ ] Can't handle not found and other error status.
## Point
- If it's necessary to import other module for test, it should be written as integration test.
- Public function should be tested in lib.rs and private function should be tested in file it located.
