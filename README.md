# odata-parser-rs
Parses OData metadata.xml documents into Rust types

---
This is an absolute bare-minimum rough implementation of the EDMX 1.0 format for parsing using serde/quick-xml into a Rust structure.

Since the purpose of this library is only to parse the metadata.xml document provided by the [Danish Parliament's OpenData endpoint](https://oda.ft.dk) it does not currently support anything not explicitly used in this document, although adding such functionality is of course welcomed.
