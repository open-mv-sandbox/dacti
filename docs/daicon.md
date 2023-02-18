# Daicon Container Format

> 🚧 *This is a working document, describing a work-in-progress format. Nothing described in this document should be seen as final. Features described in this document may not be implemented yet, or differ from as described.*

Daicon containers are a wrapping file format, made to build up flexible and extendible formats out of "components".

| Key | Value |
| --- | --- |
| Name | Daicon Container Format |
| Version | 0.1.0-draft 🚧 |

## Motivation

Daicon containers are designed, but not exclusively for, metaverse objects, and metaverse data packages. This use case presents many specific requirements that many other formats don't provide (at the same time):

- Backwards and forwards compatibility. If the design of a format changes, or a new format comes in vogue, the component system allows formats to adapt while still providing compatible components.
- Modularity and extendibility. Superset features or metadata can be added to existing formats, without requiring central coordination. This allows for new format features to be tested easily, and for adding information only relevant for one specific case, without complicating a central format specification.
- Easy to parse. Daicon containers are extremely easy to parse in any language, even without dynamic memory. The surface area of the standard is also intentionally very low, meaning no special cases or obscure extensions you need to support for full coverage.
- Low overhead. A format based on daicon containers is just 56 bytes larger than the raw component. This one bullet point is already over two times that.

Additionally, by placing additional restrictions on the binary layout of your format, you can also use the following features:

- Direct addressing. Daicon containers do not require any special parsing or decompressing at a container level to access the inner data. This is delegated to the inner components which may, in the case of "dacti packages" for example, decide to only do compression at a per-object level. This allows areas to be directly addressed through, for example, [HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests).
- Cache coherency. Daicon is designed to work well with CDN and edge caches. Derived formats can append additional data and update atomically without needing to invalidate the entire file.

## Daicon Format

Daicon containers are made up out of multiple sections.

| Bytes | Description |
| --- | --- |
| 8 | signature |
| 24 + (N * 24) | component table |
| ... | inner data |

#### Endianness

All values in the daicon specification use little-endian byte ordering. components and formats may specify different endianness in component data or inner data.

### Signature

Unless already validated by another system, implementations should start by reading the first 8 bytes, the magic signature, and validate it.

| Bytes | Description |
| --- | --- |
| 8 | signature, 0xFF followed by "daicon0" |

This should match exactly. Future incompatible versions may change "0". An implementation reading a different number there should reject the file as incompatible.

For interoperability, you should not change this signature for your own format, instead use the type UUID in the format section.

This signature starts with a non-printable character, to aide in auto-detecting daicon files as non-text files.

> 🚧 If daicon is standardized and the specification reaches 1.0 drafts, this magic prefix will be updated to enforce compatibility.

### Component Table

The component table starts with a header, describing metadata for parsing this set of components, and a pointer to the next set.

| Bytes | Description |
| --- | --- |
| 8 | extension offset |
| 4 | extension count hint |
| 4 | count |
| 8 | region offset |

Following this, you will find `count` amount of components.

| Bytes | Description |
| --- | --- |
| 16 | type UUID |
| 8 | data (typically a region) |

#### Type ID

The type ID is used to identify the location of components for compatibility and interoperability. components are expected to follow semantic versioning, with a major version increase resulting in a new ID.

Type IDs **MUST** be unique, this enforces that there is no situation where continuing to read a table will change the components already found. An implementation can decide to stop reading components early, if it has found the components it needs.

A format **MAY** specify recommended component ordering to aide in detecting the best components available for a task.

#### Data

Components define the format of their data in the table themselves, but will typically specify a "region". A "region" is an offset and size, both 4 bytes long. If specifying an offset or a region, those should be offset by the "region offset" value in the table header. When the data specifies a region, these **MAY** overlap.

> ⚠️ Always validate all offsets and sizes.

Regions are arbitrary binary data, and how they are interpreted is decided by the specific component's specification. Derived formats are encouraged to reuse standard component specifications where possible.

> ⚠️ Component regions are not required to pack tightly. A file can, and commonly will, contain much more data than just the component regions.

It is recommended for the minor version of a component to be tracked inside the component data or region. For example, as a JSON field if your component uses JSON.

#### Extension

If not null, the extension descibes the location of another component table. This is to allow the recommendations in "CDN Cache Coherency" and "Reducing Round-Trips" to be followed without limiting extensibility.

The extension count hint specifices how many components **MAY** be present at that location for efficiently pre-fetching the entire table and not just the header.

A reader **MAY** decide not to read the extension table if it has already read the components required by the format. If this is not the case, a reader **MUST** follow the extension, or inform the caller it must do so.

A reader **MUST** track tables already read, and ignore loops. A reader **MAY** raise a debugging warning when this is encountered.

Formats **MAY** opt to only include the minimal components necessary in the base table, and move all optional and less important components to an extension table, to reduce the base table size for the purpose of "Reducing Round-Trips".

### Inner Data

The rest of the file contains arbitrary data. For example:

- Component regions, containing the component implementations
- Data regions indirectly referenced by components

## Using Daicon

Daicon is intended to be used as the basis for other file formats. This allows a format to be extended, versioned with backwards compatibility, and metadata to be interpreted by common tools.

### Creating a Format

When creating a format, you should make a specification that defines which components your format **requires**, and their minimum versions. These components can be re-used between different formats, in fact, standardizing components separately is recommended. (though, not required and not always desirable)

It is recommended that you pick a unique extension, and potentially MIME-type, for your format. This gives a hint to software on how to interpret an arbitrary file. Daicon files are 'duck-typed' internally, your own format is defined by its components.

Formats intentionally do not have a file-level exclusive identifier, as this would make them mutually exclusive, which is exactly something daicon is designed to avoid.

### Creating a Component

When creating a component, you should generate a *random* UUID. This random UUID mechanism is what allows for daicon's extensibility.

You should version your component, with minor versions tracked inside your own component's data. Major versions should re-generate a new UUID.

### Versioning and Updating

Derived formats and components should use [Semantic Versioning 2.0.0](https://semver.org/), to clearly define backwards compatibility. When you add new features to your format, but maintain backwards compatibility, you should raise the minor version of your format.

A new format version can raise the minimum required version of components. The format will continue to be backwards compatible, as long as the component requirements by this new version cover the component requirements by the previous versions, following the rules set out by semantic versioning.

You can include multiple major versions of the same component in a daicon container, as they are required to have different unique UUIDs. If you find yourself needing to include multiple *minor* versions, you are likely not correctly following semantic versioning.

## Optimizing Daicon

> 🚧 This is pending to be moved to a separate optional superset specification.

### Reducing Round-Trips

If your format will be fetched *partially* from a server, and then indexed using ranges, your format specification should include recommendations to reduce necessary round-trips.

For example, you can recommend (or even require) an index component describing regions contained in your file to exist within the first 64kb. This would allow a client aware of your format to always fetch the full first 64kb and not need additional round-trips to the server.

Not all components have to fall in this region, only those that need this 'fast-path'. You are recommended to specify that clients should degrade performance rather than fail if the included components' data exceeds the specified region.

### CDN Cache Coherency

Daicon containers are designed for efficient cache coherency on CDNs and edge caches. To achieve this, daicon's component system can be updated atomically.

You can use the values in the component table as atomic switches, after appending binary data, repointing locations, and validating all caches have been updated. The component table itself also has "count" and "extension", which too can be atomically updated after verifying a cache flush.

If your format needs this functionality in combination with "Reducing Round-Trips", you are recommended to specify padding in the pre-fetch region, reserving it, to allow the file to be updated without a full cache flush. You should also pad the component table for the same reason.

### Specifying Append-Only

Binary Data previously written should **never** move or change its value to ensure stale client table requests do not retrieve corrupt data from an update. Table pointer to offsets may be updated as necessary. If a file has stale or unused sections, a new file should be created with the unnecessary data culled out.

## Examples

Examples of how to define format and component specifications on top of daicon.

> ⚠️ These are not standardized specifications, they are for educational purposes only. Do not use these for anything other than example code.

### Format Specification

This example format specification describes a file containing arbitrary text.

| Key | Value |
| --- | --- |
| Name | Text File Example |
| Version | 0.1.0 |
| UUID | 877da608-a3ae-4ca9-aae4-1bda00aedd14 |

This file format contains generic text data, to be used by text editors. The contents are stored in a "Text Component Example" component.

#### Required Components

| Name | UUID | Version |
| --- | --- | --- |
| Text Component Example | 37cb72a4-caab-440c-8b7c-869019ed348e | 0.1.0 |

#### Recommended Optional Components

| Name | UUID | Version |
| --- | --- | --- |
| Hypothetical Metadata Example | f97ca1ff-b4e6-42e0-b992-b22a3b688536 | 1.2.0 |

### Component Specification

This example component specification describes the presence of unstructured text data.

| Key | Value |
| --- | --- |
| Name | Text Component Example |
| Version | 0.1.0 |
| UUID | 37cb72a4-caab-440c-8b7c-869019ed348e |
| Table Data | Region |

The contents of the component region is UTF-8 text data. Null characters should be considered invalid data and an implementation **MUST** reject parsing the component if the region contains these.
