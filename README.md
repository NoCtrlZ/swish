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
    swish
}

fn main() {
    swish_swish().bish();
}
```
## Abstract
### Three rules🔔
There are three rules I have when making this web flamework.
- Don't depend nightly module
- Don't have extra module
- Don't be heavy
### What I want to say🏴‍☠️
Sometime other Rust web flameworks can't be built when it's compiled for production because of `dependencies with nightly modules`. Do we need to be an OSS committer? And we also don't prefer to use left-pad packages. An old coupon expired! I'd like to say `We are programmer not OSS committer nor fucking copy paster!`
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
- [ ] Version `0.1.0` Release
- [ ] Integration Test
- [ ] Refactoring According To [Here](https://github.com/NoCtrlZ/swish/blob/master/CONTRIBUTE.md)
- [ ] Define All Method And Status Code
## Test
```
$ cargo test
```