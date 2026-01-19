[package]
name = "proto_rs"
version = "0.1.0"
edition = "2024"

[dependencies]
tonic = "*"
prost = "0.14"
tonic-prost = "*"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
serde = "1.0.228"
pbjson = "0.9.0"
pbjson-types = "0.9.0"
utility-types = "0.0.4"
valid_proc = { version = "0.1.0", path = "../valid_proc" }
todouble_derive = { version = "0.1.0", path = "../todouble_derive" }
[build-dependencies]
tonic-prost-build = "*"
walkdir = "2.5.0"
tonic-build = "0.10"
tonic-buf-build = "0.1.2"
valid_proc = { version = "0.1.0", path = "../valid_proc" }
todouble_derive = { version = "0.1.0", path = "../todouble_derive" }
[features]
default = []
## @@protoc_insertion_point(features)