# Swish
![Rust Cargo Test](https://github.com/NoCtrlZ/swish/workflows/Rust%20Cargo%20Test/badge.svg?branch=master)  
This Is The Beef Web Flamework For R**ket🥩 Swish Swish Bish🎶 Another One In The Basket🏀
## Abstract
### Three rules🔔
There are three rules I have when making this web flamework.
- Don't depend nightly module
- Don't have extra module
- Don't be heavy
### What I want to say🏴‍☠️
Sometime other Rust web flameworks can't be built because it has `dependencies with nightly modules`. Do we need to be an OSS committer? It's so ridiculous! And we also don't prefer to use left-pad packages. Your game is tired! I'd like to say `We are programmer not OSS committer nor fucking copy paster!`
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
- [ ] Cros Options
- [ ] Http Request Validater
- [ ] Version `0.1.0` Release
- [ ] Define Method And Status Code
## Test
```
$ cargo test
```