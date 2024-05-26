# genson-rs

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/junyu-w/genson-rs)
[![crates.io](https://img.shields.io/crates/v/genson-rs.svg)](https://crates.io/crates/genson-rs)
[![CI](https://github.com/junyu-w/genson-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/junyu-w/genson-rs/actions/workflows/rust.yml)

*-- 🔥 Generate JSON Schema from Gigabytes of JSON data in seconds*

`genson-rs` is a Rust rewrite of the [GenSON](https://github.com/wolverdude/genson/) Python library , which can be used to generate [JSON schema](https://json-schema.org/) (Draft-04 and after) from one or multiple JSON objects.

While not having full feature parity yet, `genson-rs` focuses on **speed** ⚡️. It offers MUCH better performance (**25x ~ 75x faster**) compared to the Python `GenSON` library, and is generally a lot faster than other open source schema inference tools as well. Its high performance makes it a viable choice for online schema inference for large JSON dataset at scale. Check out the [benchmark](#benchmark) section for performance benchmark comparisons.

## Install
Installation via [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is the easiest. If you don't have it already, follow the link to set up Cargo (one simple command), then run:
```
cargo install genson-rs
```
Installing via `brew` will be supported soon.

## Usage
```
genson-rs <OPTION> <FILE(S)>
```

e.g. If you have a large JSON file full of request logs in JSON format
```
genson-rs request_logs.json
```

Additionally, if each request log is a JSON object in its own line, you can specify the delimiter which will slightly improve the performance
```
genson-rs --delimiter newline request_logs.json 
```

## Benchmark

The following benchmarks are executed manually on my local `2023 Macbook Pro with the M2 Pro Chip (10 cores, 4 high-efficiency + 6 high-performance), 16GB RAM, running macOS 13.0`. Each of the test JSON files is generated using the `json_gen.py` script inside of the `tests/data` folder, and each test was executed 3 times. The median was used out of the 3 runs.

| Library         | File Size               | Time               |
|-----------------|-------------------------|--------------------|
| GenSON (Python) | 50 MB                   | 1.61s              |
| genson-rs       | 50 MB                   | 🔥 **0.07s**       |
| GenSON (Python) | 500 MB                  | 16.07s             |
| genson-rs       | 500 MB                  | 🔥 **0.61s**       |
| GenSON (Python) | 1 GB                    | 34.21s             |
| genson-rs       | 1 GB                    | 🔥 **1.19s**       |
| GenSON (Python) | 3 GB                    | 107.86s (1min 47s) |
| genson-rs       | 3 GB                    | 🔥 **4.56s**       |
| GenSON (Python) | 3 GB (Large JSON Array) | 443.83s (7min 23s) |
| genson-rs       | 3 GB (Large JSON Array) | 🔥 **7.06s**       |

As you can see, `genson-rs` is *extremely* fast, and might be the fastest schema inference engine out there based on my rudimentary benchmarks against other tools (that I'm aware of) as well.

## Optimization Techniques 

The `genson-rs` library leverages the following techniques to greatly speed up the schema generation process:
- ⚡️ **Rust being blazingly fast itself** -- without any GC or interpreter overhead, a 1-to-1 port in Rust running on a single CPU core runs 2x faster than the Python version already
- ⚡️ **Parallel processing leveraging all available CPU cores** -- whie Python has the limitation of the GIL that prevents it from leveraging multiple CPU cores efficiently, `genson-rs` parallelizes [Map-Reduce](https://en.wikipedia.org/wiki/MapReduce) type of workload whenever possible (e.g. when processing gigantic arrays), maxing out all the available CPU cores
- ⚡️ **Extremely fast JSON parsing powered by SIMD instructions** -- instead of fully parsing out the whole JSON dataset, we use the `simd-json` library (a Rust port of the C++ `simdjson` library) that leverages SIMD (Single Instruction/Multiple Data) instructions to only parse out the "tape" of the JSON dataset, which is sufficient enough to build the schema on top of without fully deserializing the whole dataset
- ⚡️ **Efficient memory management using the MiMalloc allocator** -- this is recommended by the `simd-json` library itself, `genson-rs` opts to use the `MiMalloc` allocator instead of the default global allocator which made the code run faster by a decent amount
