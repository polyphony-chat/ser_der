# serder

Newtypes for the der crate to allow for [serde](https://serde.rs) de-/serialization.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
serder = "0"
der = { version = "0", features = ["alloc"] }
serde = { version = "1", features = ["derive"] }
```

You can then use the newtypes offered by this crate to de-/serialize DER-encoded data structures using serde.

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct MyStruct {
    #[serde(with = "serder::asn1::ia5_string")]
    bits: serder::asn1::IA5String,
}
```

All newtypes implement `Deref`, `DerefMut`, `From<[Newtype]> for [Type]` and `From<[Type]> for [Newtype]` to the underlying `der` type.

## Upstreaming to `der`

This crate is a temporary solution until the `der` crate supports serde de-/serialization. The plan is to upstream the serde support to the [`der` crate](https://github.com/RustCrypto/formats/tree/master/der).