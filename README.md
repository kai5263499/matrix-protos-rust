# matrix-protos-rust

Rust bindings for the [matrix io](https://github.com/matrix-io/protocol-buffers)

## Building the bindings

There are two ways to build the protobuf bindings.

### The protoc way

The first is by using the Makefile which depends on the [protobuf rust codegen binary](https://crates.io/crates/protobuf-codegen). This binary can be installed with cargo
```
cargo install protobuf-codegen
```

This also requires the [protoc](https://grpc.io/docs/protoc-installation/) binary to be installed.

Once the two binaries are installed, you can download the protos from the matrix-io project using `make protocol-buffers-master.zip`

And then build the bindings with `make bindings`

### The native Rust way

The second is by using the cargo build helper along with the [protobuf-codegen-pure](https://crates.io/crates/protobuf-codegen-pure) crate.

To build the bindings you can simply run the cargo command `cargo build`

This still requires/assumes that the matrix-io protos are avaliable in the protocol-buffers-master directory. So it still makes sense to run the `make protocol-buffers-master.zip` command mentioned above prior to `cargo build`

## Usage

TODO

## Example

Examples of using these bindings are available in the examples directory.