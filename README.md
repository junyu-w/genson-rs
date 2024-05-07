# genson-rs

`genson-rs` is a **Rust rewrite** of the python [GenSON](https://github.com/wolverdude/genson/) library that derives JSON schemas for the input JSON objects (or schemas).

`genson-rs` aims to provide much better performance compared to the python version through the following techniques:
- use a [streaming deserializer](https://docs.rs/serde_json/latest/serde_json/struct.StreamDeserializer.html) offered by `serde` to not having to parse all objects at once
- `rust` being faster than python in general with no GC or interpreter overhead
- true parallelism utilizing all CPU cores without the notorious python GIL

## TODOs:
- [x] This is slow AF -- (50mb json, genson takes 1.8s, this takes 6.2s. wtf)
- [x] fix the scalar string type double escape issue
- [x] implement the tuple strategy