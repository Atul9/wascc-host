# waSCC Host Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2020 JUN 8

This release was primarily to accomodate the upgrade to the newest version of the [waSCC Codec](../wascc-codec).

### Changed

All capability providers (including _portable_ WASI providers) are now required to respond to the operation `OP_GET_CAPABILITY_DESCRIPTOR` and return a messagepack-serialized struct containing metadata about the capability provider. This metadata includes:

* Name
* Documentation description
* Version (semver string) and Revision (monotonic)
* List of supported operations

We created a simple _builder_ syntax that makes it easy and readable for capability providers to supply a capability descriptor:

```rust
/// Obtains the capability provider descriptor
fn get_descriptor(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(serialize(
        CapabilityDescriptor::builder()
            .id(CAPABILITY_ID)
            .name("Default waSCC HTTP Server Provider (Actix)")
            .long_description("A fast, multi-threaded HTTP server for waSCC actors")
            .version(VERSION)
            .revision(REVISION)
            .with_operation(
                OP_HANDLE_REQUEST,
                OperationDirection::ToActor,
                "Delivers an HTTP request to an actor and expects an HTTP response in return",
            )
            .build(),
    )?)
}
```

**NOTE** - This is a breaking change, so old versions of capability providers will _not_ work with this version of the waSCC host.
