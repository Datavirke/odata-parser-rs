# odata-parser-rs [![Release] ![Latest Version]][crates.io] [![Docs]][docs.rs]

[Release]: https://github.com/Datavirke/odata-parser-rs/actions/workflows/release.yml/badge.svg?event=push

[Latest Version]: https://img.shields.io/crates/v/odata-parser-rs
[crates.io]: https://crates.io/crates/odata-parser-rs
[Docs]: https://docs.rs/odata-parser-rs/badge.svg
[docs.rs]: https://docs.rs/odata-parser-rs

Deserializes OData 2.0 metadata.xml documents.

---
This is an absolute bare-minimum rough implementation of the EDMX 1.0 format for parsing using serde/quick-xml into a Rust structure.

Since the purpose of this library is only to parse the metadata.xml document provided by the [Danish Parliament's OpenData endpoint](https://oda.ft.dk) it does not currently support anything not explicitly used in this document, although adding such functionality is of course welcomed.

# Example

Parse an example `metadata.xml` file, and print all the `EntitySets` within the default schema.
```rust
let edmx = Edmx::from_str(include_str!("my-metadata.xml")).unwrap();
let schema = edmx.default_schema().unwrap();

for entity_set in schema.entity_sets().unwrap() {
  println!("{:#?}", entity_set);
}
```

Using the [test file](tests/folketinget.xml) from the Danish Parliament, you should see output similar to this:
```
EntitySet {
    name: "Afstemning",
    entity_type: "FT.Domain.Models.Afstemning",
}
EntitySet {
    name: "Afstemningstype",
    entity_type: "FT.Domain.Models.Afstemningstype",
}

(... and so on)
```