# Daicon Container Format

> 🚧 *This is a working document, describing a work-in-progress format. Nothing described in this document should be seen as final. Features described in this document may not be implemented yet, or differ from as described.*

Daicon containers are a wrapping file format, made to make file self-description and versioning easier. They let a file format describe itself using a UUID and semantic version. Additionally, they provide a flexible way to define named and versioned regions of data in the file, called "interfaces".

| | |
| --- | --- |
| Name | Daicon Container Format |
| Version | 0.1.0-draft 🚧 |

## Motivation

Daicon containers are designed, but not exclusively for, containing metaverse objects and object data in a way that allows flexible interoperability and direct addressing. This use case presents a few requirements that many other formats don't provide:

- Backwards and forwards compatibility. If the design of a format changes, or a new format comes in vogue, the interface system allows formats to adapt while still providing interfaces for older systems.
- Easy to parse. Daicon containers are extremely easy to parse in any language, even without dynamic memory. The surface area of the standard is also intentionally very low, meaning no special cases or obscure extensions you need to support for full coverage.
- Low overhead. A format based on daicon containers is just 36 bytes larger than the raw format itself. This one bullet point alone is over four times as large as that.
- Direct addressing. Daicon containers do not require any special parsing or decompressing at a container level to access the inner data. This is delegated to the inner interfaces which may, in the case of "dacti packages" for example, decide to only do compression at a per-object level. This allows areas to be directly addressed through, for example, [HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests).
- Inner type metdata and versioning. Besides identifying and versioning interfaces, a format that uses daicon containers can also be uniquely identified by the header, including backwards and forwards compatibility for minor versions.
- Gradual region invalidation, even across base format updates. Daicon allows you to relocate the interface table to any location, this means you can dynamically allocate space as required without being forced to collide with data in old regions. This is useful for caching, that old data may still be in use!

## Using Daicon for a Format

Daicon is intended to be used as the basis for other file formats. This allows a format to be extended, versioned with backwards compatibility, and metadata to be interpreted by common tools.

### Creating a Format

When you use daicon containers for your format, you need to randomly generate a UUID to identify your format with.

Additionally, it is recommended that you pick an extension for your file. Daicon files, while allowing standardized parsing of certain metadata, are not intended to contain more than one base format.

You should then define which interfaces, and their minimum versions, your format **requires**. These interfaces can be re-used between different formats, in fact, the use of standard interfaces is recommended.

### Updating a Format

If you create a new version of your format, you need to follow semantic versioning. This means that if you increase the minor version but not the major version, software created for the previous minor versions should be able to load the file.

For example, this means you can increase the minimum minor version of an interface. But, you cannot increase the required major version, or decrease the required minor version of an interface. If you want to do this anyways, you will have to add it as an *additional* requirement, in addition to the old one.

To allow this method of backwards compatibility, you are allowed to include multiple major versions of interfaces. If you find yourself needing to include multiple *minor* versions, you are likely not correctly following semantic versioning.

### Early Partial Loading

A format may define a *recommended* order of interfaces in the file, and recommend the interface table to be placed at the start. This should never be required, however.

A conforming implementation may take advantage of recommended ordering to reduce time-to-render by interpreting the early incomplete data first, before the rest has arrived.

For example, an especially large multimedia objects index file, with optional 'levels of detail', may have the indices and cross-references for the lowest levels of detail first.

This would allow an implementation fetching additional data to start fetching related objects before the entire index file has arrived, reducing head-of-line blocking issues.

## Daicon Format

Daicon container files are made up out of multiple sections.

| Bytes | Description |
| --- | --- |
| 12 | magic header |
| 28 | format section |
| ... | inner data |

Additionally, the interface table can exist at any location in the inner data.

| Bytes | Description |
| --- | --- |
| 4 | header |
| N * 36 | interface entries |

### Magic Header

Unless already validated by another system, implementations should start by reading the first 8 bytes, the magic header section section, and validating it.

| Bytes | Description |
| --- | --- |
| 8 | "daicon00" magic prefix |

This should match exactly. Future incompatible versions may change "00". For interoperability reasons, you should not change this header for your own format, instead use the type UUID in the format section.

### Format Section

| Bytes | Description |
| --- | --- |
| 16 | type UUID |
| 2 | version major |
| 2 | version minor |
| 8 | interface table index |

You can see the type UUID here as equivalent to an inner MIME-Type. Formats that use daicon containers have file extensions and MIME-Types of their own. Daicon containers are intended to be used as the base of another format.

The interface table offset is used to find the specific location of the interface table. This allows updated interface tables to be gradually introduced without invalidating the entire file.

### Interface Table

A short header defines how many interfaces will be described.

| Bytes | Description |
| --- | --- |
| 4 | count |

Following this, you will find `count` amount of interfaces.

| Bytes | Description |
| --- | --- |
| 16 | type UUID |
| 2 | version major |
| 2 | version minor |
| 8 | offset |
| 8 | size |

The offset and size describe the location of the interface in the file. Interface regions **MAY** overlap.

> ⚠️ Always validate all offsets and sizes.

Multiple entries with the same UUID are valid, as long as their *major* versions are different. Multiple entries with the same UUID *and* major version are not valid, and **MUST** be rejected by a parser implementation.

Interfaces are arbitrary binary data, and how they are interpreted is decided by the specific format using daicon containers. Derived formats are encouraged to reuse standard interface specifications where possible.

> ⚠️ Interfaces are not required to pack tightly. A file can, and commonly will, contain much more data than just the interface data regions.

### Inner Data

After these sections, the rest of the file contains arbitrary data. For example:

- Interface regions, containing the interface implementations
- Data regions indirectly referenced by interfaces
- Extended data used by future versions of this specification

## Glossary

### Semantic Versioning

Version numbers in this standard use [Semantic Versioning 2.0.0](https://semver.org/). For example, an interface implementation that supports reading an interface format version of at least version "2.1", can read an interface format version of "2.4", but not "3.0" or "2.0". The "patch" version is not included, as this refers to bug-fixes, and should only be used to version an interface's specification.

> A package version with a major of "0" should always be assumed to be mutually-incompatible with other minor versions. Implementations against these versions of this specification or interfaces should verify supported minor versions individually.
