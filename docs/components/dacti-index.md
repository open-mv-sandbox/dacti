# Dacti Index Component

> 🚧 *This is a working document, describing a work-in-progress format. Nothing described in this document should be seen as final. Features described in this document may not be implemented yet, or differ from as described.*

Dacti index component is a daicon component that lists regions of data, indexable by UUID, with optional compression.

Dacti indices intentionally support a very limited feature set, to make it very easy to support a base level of package loading.

If you need more extended features, like file paths for data, you should create an optional extension daicon component for this purpose.

| Key | Value |
| --- | --- |
| Name | Dacti Index Component |
| Version | 0.1.0-draft 🚧 |
| Daicon Version | 0.1.0 |
| UUID | 2c5e4717-b715-429b-85cd-d320d242547a |
| Table Data | Region |

## Format

| Bytes | Description |
| --- | --- |
| 8 | Header |
| N * (16 + 255 * 24) | Groups |

### Header

| Bytes | Description |
| --- | --- |
| 4 | Version |
| 4 | Groups |

The version contains the minor version of this specification, following semantic versioning.

> 🚧 During pre-1.0, the version value will always be 0.

### Group Header

| Bytes | Description |
| --- | --- |
| 8 | Entries Offset |
| 4 | Encoding |
| 1 | Length |
| 3 | Reserved Padding |

The group header is always followed by 255 entries of space, before the next group header starts. The "Length" determines the amount of entires that are valid to be read.

### Encoding

Encodings are stored as a 4-byte UTF-8 string, padded with nulls at the end.

| Value | Description |
| --- | --- |
| none | No Encoding |
| brot | Brotli Compression |

### Entry

| Bytes | Description |
| --- | --- |
| 16 | Region ID |
| 4 | Offset |
| 4 | Size |

The "Region ID" is a UUID that identifies the region. This ID is used for implementations to look up specific sections of data.

> ⚠️ Always validate the package is a valid origin for an ID's data. Packages have no internal validation mechanism to prove point of origin. For example, if origins aren't validated, a script served by a package could run in elevated permissions due to intentionally sharing an ID with a trusted script, despite being fetched form the wrong (malicious) package.

## Change Log

### 0.1.0-draft 🚧

- Initial publication
