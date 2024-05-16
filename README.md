# genson-rs

*-- Generate JSON Schema from Gigabytes of JSON files in seconds*

`genson-rs` is a port of the python [GenSON](https://github.com/wolverdude/genson/) library written in Rust, which can be used to generate JSON schema (Draft-04 and after) from one or multiple JSON objects.

Instead of feature parity, `genson-rs` focuses on **speed and scale**, it aims to offer much better performance (10x) compared to the python version and thus supports schema generation from much larger JSON files. See the [benchmark](#benchmark) section for performance comparisons with the python `GenSON` library and other popular json schema generation tools.

Note that `genson-rs` currently only supports schema generation from JSON objects, it is not the goal of the project at the moment to achieve feature parity with the `GenSON` library. I would recommend to check it out if you have more complex schema generation needs.

## Benchmark

The following benchmarks are executed manually on an EC2 instance with : <TBD - spec>

TBD - table here

## Optimization Techniques

The `genson-rs` library leverages the following techniques to greatly speed up the schema generation process compared to the python version:
- **Rust being blazingly fast itself** -- without any GC and interpreter overhead, a 1-to-1 port in Rust running on a single CPU core runs 2x faster than the Python version already
- **True parallelism on multi-core CPU** -- whie Python has the limitation of the GIL that prevents it from leveraging multiple CPU cores efficiently, the Rust code processes large map-reduce type of workload (e.g. when processing gigantic arrays) in parallel on all the available CPU cores to greatly speed up the schema building time
- **Extremely fast JSON parsing built upon SIMD instructions** -- instead of fully parsing out the whole JSON structure, the Rust code uses the `simd-json` library (a Rust port of the C++ `simdjson` library) that leverages the SIMD (Single Instruction/Multiple Data) instructions and a two-pass algorithm to parse out the "tape" of the JSON dataset, which is sufficient enough to build the schema on top without fully deserializing the whole dataset
- **Efficient memory management using the MiMalloc allocator** -- this is recommended by the `simd-json` library itself, the Rust code opts to use the `MiMalloc` allocator instead of the default global allocator which made the code run faster by a decent amount

