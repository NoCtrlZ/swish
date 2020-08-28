# Swish
[![crates.io badge](https://img.shields.io/crates/v/swish_swish.svg)](https://crates.io/crates/swish_swish)
![Build Test](https://github.com/NoCtrlZ/swish/workflows/Rust%20Cargo%20Test/badge.svg?branch=master)  
This Is The Beef Web Flamework For R**ket🥩 Swish Swish Bish🎶 Another One In The Basket🏀
```rust
extern crate swish_swish;

use swish_swish::*;

fn swish_swish() -> Swish {
    let mut swish = Swish::new();
    swish.get("/path", path_handler);
    swish.get("/user/:id", user_id_handler);
    swish.post("/user/register", user_register_handler);
    swish.set_cors_as(allow_everything());
    swish
}

fn main() {
    swish_swish().bish();
}
```
Full Demo Application Is [Here](https://github.com/NoCtrlZ/swish/blob/master/test/server)🎱
## Abstract
### Three rules🔔
There are three rules when making this web flamework.
- Don't be complicated
- Don't implement extra function
- Don't depend with nightly module

### What I want to say🏴‍☠️
Sometime other Rust web flameworks can't be built when it's compiled for production because of `dependencies with nightly modules`. Do we need to be an OSS committer? And we also don't prefer to use left-pad packages. R**ket is about as useful as an old coupon expired! I'd like to say `We are programmer not OSS committer nor fucking copy paster!`
## Installation
```
[dependencies]
swish_swish = "0.1.6"
```
## Dependencies Except Std
I count the crime I committed. Karma‘s not a liar!
- regex
- serde
- serde_json
## Roadmap
- [x] Basic Test Server
- [x] Client For Test
- [x] Route Register
- [x] Handler And Route Matcher
- [x] Static Route Matcher
- [x] Dynamic Route Matcher
- [x] Design Request And Response Struct
- [x] Design Error Responser
- [x] Dynamic Route Param Getter
- [x] Actual Http Compiler
- [x] Actual Http Responser
- [x] Handler Returns Only Body With Chosing Content Type
- [x] Support Post Request
- [x] Return Basic Header Like Content Lenght
- [x] Cros Options
- [x] Http Request Validater
- [x] Version `0.1.0` Release
- [x] Integration Test
- [x] Anti Pattern Test
- [x] Integrate Cors, Config And Request, Header
- [ ] Unit Test
- [ ] Refactoring According To [Here](https://github.com/NoCtrlZ/swish/blob/master/CONTRIBUTE.md)
- [ ] Get Method And Path From Attribute
- [ ] Version `1.0.0` Release
- [ ] Define All Method And Status Code
## Test
```
$ cargo test
```
