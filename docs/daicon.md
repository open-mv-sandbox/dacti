# Daicon Container Format

> 🚧 *This is a working document, describing a work-in-progress format. Nothing described in this document should be seen as final. Features described in this document may not be implemented yet, or differ from as described.*

Daicon containers are a wrapping file format, made to make file self-description and versioning easier. They let a file format describe its features using extendable and versioned "interfaces".

| Key | Value |
| --- | --- |
| Name | Daicon Container Format |
| Version | 0.1.0-draft 🚧 |

## Motivation

Daicon containers are designed, but not exclusively for, metaverse objects, and metaverse data packages. This use case presents many specific requirements that many other formats don't provide:

- Backwards and forwards compatibility. If the design of a format changes, or a new format comes in vogue, the interface system allows formats to adapt while still providing compatible interfaces.
- Modularity and extendibility. Superset features or metadata can be added to existing formats, without requiring central coordination. This allows for new format features to be tested easily, and for adding information only relevant for one specific case, without complicating a central format specification.
- Easy to parse. Daicon containers are extremely easy to parse in any language, even without dynamic memory. The surface area of the standard is also intentionally very low, meaning no special cases or obscure extensions you need to support for full coverage.
- Low overhead. A format based on daicon containers is just 56 bytes larger than the raw interface. This one bullet point is already over two times that.

Additionally, by placing additional restrictions on the binary layout of your format, you can also use the following features:

- Direct addressing. Daicon containers do not require any special parsing or decompressing at a container level to access the inner data. This is delegated to the inner interfaces which may, in the case of "dacti packages" for example, decide to only do compression at a per-object level. This allows areas to be directly addressed through, for example, [HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests).
- Cache coherency. Daicon is designed to work well with CDN and edge caches. Derived formats can append additional data and update atomically without needing to invalidate the entire file.

## Daicon Format

Daicon containers are made up out of multiple sections.

| Bytes | Description |
| --- | --- |
| 8 | signature |
| 24 + (N * 24) | interface table |
| ... | inner data |

#### Endianness

All values in the daicon specification use little-endian byte ordering. Interfaces and formats may specify different endianness in interface data or inner data.

### Signature

Unless already validated by another system, implementations should start by reading the first 8 bytes, the magic signature, and validate it.

| Bytes | Description |
| --- | --- |
| 8 | signature, 0xFF followed by "daicon0" |

This should match exactly. Future incompatible versions may change "0". An implementation reading a different number there should reject the file as incompatible.

For interoperability, you should not change this signature for your own format, instead use the type UUID in the format section.

This signature starts with a non-printable character, to aide in auto-detecting daicon files as non-text files.

> 🚧 If daicon is standardized and the specification reaches 1.0 drafts, this magic prefix will be updated to enforce compatibility.

### Interface Table

The interface table starts with a header, describing metadata for parsing this set of interfaces, and a pointer to the next set.

| Bytes | Description |
| --- | --- |
| 8 | extension offset |
| 4 | extension count hint |
| 4 | count |
| 8 | region offset |

Following this, you will find `count` amount of interfaces.

| Bytes | Description |
| --- | --- |
| 16 | type UUID |
| 8 | data (typically a region) |

#### Type ID

The type ID is used to identify the location of interfaces for compatibility and interoperability. Interfaces are expected to follow semantic versioning, with a major version increase resulting in a new ID.

Type IDs **MUST** be unique, this enforces that there is no situation where continuing to read a table will change the interfaces already found. An implementation can decide to stop reading interfaces early, if it has found the interfaces it needs.

A format **MAY** specify recommended interface ordering to aide in detecting the best interfaces available for a task.

#### Extension

If not null, the extension descibes the location of another interface table. This is to allow the recommendations in "CDN Cache Coherency" and "Reducing Round-Trips" to be followed without limiting extensibility.

The extension count hint specifices how many interfaces **MAY** be present at that location for efficiently pre-fetching the entire table and not just the header.

A reader **MAY** decide not to read the extension table if it has already read the interfaces required by the format. If this is not the case, a reader **MUST** follow the extension, or inform the caller it must do so.

A reader **MUST** track tables already read, and ignore loops. A reader **MAY** raise a debugging warning when this is encountered.

Formats **MAY** opt to only include the minimal interfaces necessary in the base table, and move all optional and less important interfaces to an extension table, to reduce the base table size for the purpose of "Reducing Round-Trips".

#### Data

Interfaces define the format of their data in the table themselves, but will typically specify a "region". A "region" is an offset and size, both 4 bytes long. If specifying an offset or a region, those should be offset by the "region offset" value in the table header. When the data specifies a region, these **MAY** overlap.

> ⚠️ Always validate all offsets and sizes.

Regions are arbitrary binary data, and how they are interpreted is decided by the specific interface's specification. Derived formats are encouraged to reuse standard interface specifications where possible.

> ⚠️ Interface regions are not required to pack tightly. A file can, and commonly will, contain much more data than just the interface regions.

### Inner Data

The rest of the file contains arbitrary data. For example:

- Interface regions, containing the interface implementations
- Data regions indirectly referenced by interfaces

## Using Daicon

Daicon is intended to be used as the basis for other file formats. This allows a format to be extended, versioned with backwards compatibility, and metadata to be interpreted by common tools.

### Creating a Format

When creating a format, you should make a specification that defines which interfaces your format **requires**, and their minimum versions. These interfaces can be re-used between different formats, in fact, standardizing interfaces separately is recommended. (though, not required and not always desirable)

It is recommended that you pick a unique extension, and potentially MIME-type, for your format. This gives a hint to software on how to interpret an arbitrary file. Daicon files are 'duck-typed' internally, your own format is defined by its interfaces.

Formats intentionally do not have a file-level exclusive identifier, as this would make them mutually exclusive, which is exactly something daicon is designed to avoid.

### Creating an Interface

When creating an interface, you should generate a *random* UUID. This random UUID mechanism is what allows for daicon's extensibility.

You should version your interface, with minor versions tracked inside your own interface's data. Major versions should re-generate a new UUID.

### Versioning and Updating

Derived formats and interfaces should use [Semantic Versioning 2.0.0](https://semver.org/), to clearly define backwards compatibility. When you add new features to your format, but maintain backwards compatibility, you should raise the minor version of your format.

A new format version can raise the minimum required version of interfaces. The format will continue to be backwards compatible, as long as the interface requirements by this new version cover the interface requirements by the previous versions, following the rules set out by semantic versioning.

You can include multiple major versions of the same interface in a daicon container, as they are required to have different unique UUIDs. If you find yourself needing to include multiple *minor* versions, you are likely not correctly following semantic versioning.

### Reducing Round-Trips

> 🚧 This is pending to be moved to a separate optional superset specification.

If your format will be fetched *partially* from a server, and then indexed using ranges, your format specification should include recommendations to reduce necessary round-trips.

For example, you can recommend (or even require) an index interface describing regions contained in your file to exist within the first 64kb. This would allow a client aware of your format to always fetch the full first 64kb and not need additional round-trips to the server.

Not all interfaces have to fall in this region, only those that need this 'fast-path'. You are recommended to specify that clients should degrade performance rather than fail if the included interfaces' data exceeds the specified region.

### CDN Cache Coherency

> 🚧 This is pending to be moved to a separate optional superset specification.

Daicon containers are designed for efficient cache coherency on CDNs and edge caches. To achieve this, daicon's interface system can be updated atomically.

You can use the values in the interface table as atomic switches, after appending binary data, repointing locations, and validating all caches have been updated. The interface table itself also has "count" and "extension", which too can be atomically updated after verifying a cache flush.

If your format needs this functionality in combination with "Reducing Round-Trips", you are recommended to specify padding in the pre-fetch region, reserving it, to allow the file to be updated without a full cache flush. You should also pad the interface table for the same reason.

### Specifying Append-Only

> 🚧 This is pending to be moved to a separate optional superset specification.

Binary Data previously written should **never** move or change its value to ensure stale client table requests do not retrieve corrupt data from an update. Table pointer to offsets may be updated as necessary. If a file has stale or unused sections, a new file should be created with the unnecessary data culled out.

## Examples

Examples of how to define format and interface specifications on top of daicon.

> ⚠️ These are not standardized specifications, they are for educational purposes only. Do not use these for anything other than example code.

### Format Specification

This example format specification describes a file containing arbitrary text.

| Key | Value |
| --- | --- |
| Name | Text File Example |
| Version | 0.1.0 |
| UUID | 877da608-a3ae-4ca9-aae4-1bda00aedd14 |

This file format contains generic text data, to be used by text editors. The contents are stored in a "Text Interface Example" interface.

#### Required Interfaces

| Name | UUID | Version |
| --- | --- | --- |
| Text Interface Example | 37cb72a4-caab-440c-8b7c-869019ed348e | 0.1.0 |

#### Recommended Optional Interfaces

| Name | UUID | Version |
| --- | --- | --- |
| Hypothetical Metadata Example | f97ca1ff-b4e6-42e0-b992-b22a3b688536 | 1.2.0 |

### Interface Specification

This example interface specification describes the presence of unstructured text data.

| Key | Value |
| --- | --- |
| Name | Text Interface Example |
| Version | 0.1.0 |
| UUID | 37cb72a4-caab-440c-8b7c-869019ed348e |
| Table Data | Region |

The contents of the interface region is UTF-8 text data. Null characters should be considered invalid data and an implementation **MUST** reject parsing the interface if the region contains these.
