# Daicon Container Format

> 🚧 *This is a working document, describing a work-in-progress format. Nothing described in this document should be seen as final. Features described in this document may not be implemented yet, or differ from as described.*

Daicon containers are a wrapping file format, made to make file self-description and versioning easier. They let a file format describe itself using a UUID and semantic version. Additionally, they provide a flexible way to define named and versioned regions of data in the file, called "interfaces".

| Key | Value |
| --- | --- |
| Name | Daicon Container Format |
| Version | 0.1.0-draft 🚧 |

## Motivation

Daicon containers are designed, but not exclusively for, containing metaverse objects and object data in a way that allows flexible interoperability and direct addressing. This use case presents a few requirements that many other formats don't provide:

- Backwards and forwards compatibility. If the design of a format changes, or a new format comes in vogue, the interface system allows formats to adapt while still providing interfaces for older systems.
- Easy to parse. Daicon containers are extremely easy to parse in any language, even without dynamic memory. The surface area of the standard is also intentionally very low, meaning no special cases or obscure extensions you need to support for full coverage.
- Low overhead. A format based on daicon containers is just 80 bytes larger than the raw interface. This one bullet point alone is already a bit over two times that.
- Inner type metdata and versioning. Besides identifying and versioning interfaces, a format that uses daicon containers can also be uniquely identified by the header, including backwards and forwards compatibility for minor versions.
- Direct addressing. Daicon containers do not require any special parsing or decompressing at a container level to access the inner data. This is delegated to the inner interfaces which may, in the case of "dacti packages" for example, decide to only do compression at a per-object level. This allows areas to be directly addressed through, for example, [HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests).
- Cache coherency. Daicon is designed to work well with CDN and edge caches. Derived formats can append additional data and update atomically without needing to invalidate the entire file.

## Using Daicon for a Format

Daicon is intended to be used as the basis for other file formats. This allows a format to be extended, versioned with backwards compatibility, and metadata to be interpreted by common tools.

### Creating a Format

When you use daicon containers for your format, you need to randomly generate a UUID to identify your format with. It is recommended that you pick a unique extension for your file.

You should then define which interfaces, and their minimum versions, your format **requires**. These interfaces can be re-used between different formats, in fact, standardizing interfaces separately is recommended.

### Updating a Format

If you create a new version of your format, you need to follow semantic versioning. This means that if you increase the minor version but not the major version, software created for the previous minor versions should be able to load the file.

For example, this means you can increase the minimum minor version of an interface. But, you cannot increase the required major version, or decrease the required minor version of an interface. If you want to do this anyways, you will have to add it as an *additional* requirement, in addition to the old one.

To allow this method of backwards compatibility, you are allowed to include multiple major versions of interfaces. If you find yourself needing to include multiple *minor* versions, you are likely not correctly following semantic versioning.

### Reducing Round-Trips

If your format will be fetched *partially* from a server, and then indexed using ranges, your format specification should include recommendations to reduce necessary round-trips.

For example, you can recommend (or even require) an index interface describing regions contained in your file to exist within the first 64kb. This would allow a client aware of your format to always fetch the full first 64kb and not need additional round-trips to the server.

Not all interfaces have to fall in this region, only those that need this 'fast-path'. You are recommended to specify that clients should degrade performance rather than fail if the included interfaces' data exceeds the specified region.

### CDN Cache Coherency

Daicon containers are designed for efficient cache coherency on CDNs and edge caches. To achieve this, daicon's interface system can be updated atomically.

You can use the values in the interface table as atomic switches, after appending binary data, repointing locations, and validating all caches have been updated. The interface table itself also has "count" and "extension", which too can be atomically updated after verifying a cache flush.

If your format needs this functionality in combination with "Reducing Round-Trips", you are recommended to specify padding in the pre-fetch region, reserving it, to allow the file to be updated without a full cache flush. You should also pad the interface table for the same reason.

### Specifying Append-Only

Binary Data previously written should **never** move or change its value to ensure stale client table requests do not retrieve corrupt data from an update. Table pointer to offsets may be updated as necessary. If a file has stale or unused sections, a new file should be created with the unnecessary data culled out.

## Daicon Format

Daicon containers are made up out of multiple sections.

| Bytes | Description |
| --- | --- |
| 8 | signature |
| 20 | format header |
| 24 + (N * 28) | interface table |
| ... | inner data |

#### Endianness

All values in the daicon specification use little-endian byte ordering. Interfaces and formats may specify different endianness in interface data or inner data.

### Signature

Unless already validated by another system, implementations should start by reading the first 8 bytes, the magic signature, and validate it.

| Bytes | Description |
| --- | --- |
| 8 | signature, 0xFF followed by "daicon0" |

This should match exactly. Future incompatible versions may change "0". An implementation reading a different number there should reject the file as incompatible.

For interoperability reasons, you should not change this signature for your own format, instead use the type UUID in the format section.

> This signature starts with a non-printable character, to aide in auto-detecting daicon files as non-text files.

> 🚧 If daicon is standardized and the specification reaches 1.0 drafts, this magic prefix will be updated to enforce compatibility.

### Format Header

| Bytes | Description |
| --- | --- |
| 16 | type UUID |
| 2 | version major |
| 2 | version minor |

The type UUID is equivalent to an inner MIME-Type. Formats that use daicon containers have file extensions and MIME-Types of their own, it is repeated here for validation, and for if this information is not otherwise available.

### Interface Table

The interface table starts with a header, describing metadata for parsing this set of interfaces, and a pointer to the next set.

| Bytes | Description |
| --- | --- |
| 8 | region offset |
| 8 | extension offset |
| 4 | reserved (currently padding, write zero) |
| 4 | count |

Following this, you will find `count` amount of interfaces.

| Bytes | Description |
| --- | --- |
| 16 | type UUID |
| 2 | version major |
| 2 | version minor |
| 8 | data (typically a region) |

Interfaces define the format of their data in the table themselves, but will typically specify a "Region", defined by an offset and size, both 4 bytes long. If specifying an offset or a region, those should be offset by the "region offset" value in the table header. When the data specifies a region, these **MAY** overlap.

> ⚠️ Always validate all offsets and sizes.

Regions are arbitrary binary data, and how they are interpreted is decided by the specific format using daicon containers. Derived formats are encouraged to reuse standard interface specifications where possible.

> ⚠️ Interface regions are not required to pack tightly. A file can, and commonly will, contain much more data than just the interface regions.

#### Duplicates

Multiple entries with the same UUID are distinct, as long as their *major* versions are different. Multiple entries with the same UUID *and* major version are not valid, and a reader **MUST** ignore duplicate entries later in the table.

This **MUST** enforce that there is no situation where continuing to read a table will change the interfaces already found. An implementation can decide to stop reading interfaces early, if it has found the interfaces it needs.

A format **MAY** specify recommended interface ordering to aide in detecting the best interfaces available for a task.

#### Extension

If not null, the extension descibes the location of another interface table. This is to allow the recommendations in "CDN Cache Coherency" and "Reducing Round-Trips" to be followed without limiting extensibility.

A reader **MAY** decide not to read the extension table if it has already read the interfaces required by the format. If this is not the case, a reader **MUST** follow the extension, or inform the caller it must do so.

A reader **MUST** track tables already read, and ignore loops. A reader **MAY** raise a debugging warning when this is encountered.

Formats **MAY** opt to only include the minimal amount of interfaces necessary in the base table, and move all optional and less important interfaces to an extension table, to reduce the base table size for the purpose of "Reducing Round-Trips".

### Inner Data

After these sections, the rest of the file contains arbitrary data. For example:

- Interface regions, containing the interface implementations
- Data regions indirectly referenced by interfaces

## Examples

Examples of how to define format and interface specifications on top of dacti.

> ⚠️ These are not standardized specifications, do not use these.

### Interface Specification

This example interface specification describes the presence of unstructured generic text data.

| Key | Value |
| --- | --- |
| Name | Text Example |
| Version | 0.1.0-draft 🚧 |
| UUID | 37cb72a4-caab-440c-8b7c-869019ed348e |
| Table Data | Region |

The contents of the interface region is UTF-8 text data. Null characters should be considered invalid data and an implementation **MUST** reject parsing the interface if the region contains these.

## Glossary

### Semantic Versioning

Version numbers in this standard use [Semantic Versioning 2.0.0](https://semver.org/). For example, an interface implementation that supports reading an interface format version of at least version "2.1", can read an interface format version of "2.4", but not "3.0" or "2.0". The "patch" version is not included, as this refers to bug-fixes, and should only be used to version an interface's specification.

> A package version with a major of "0" should always be assumed to be mutually-incompatible with other minor versions. Implementations against these versions of this specification or interfaces should verify supported minor versions individually.
