# Dacti Package Format

> 🚧 *This is a working document, describing a work-in-progress format. Nothing described in this document should be seen as final. Features described in this document may not be implemented yet, or differ from as described.*

Dacti packages are a general-purpose data container, designed for fetching subsets of data from large repositories, and for packaging data in files.

| Key | Value |
| --- | --- |
| Name | Dacti Package Format |
| Version | 0.1.0-draft 🚧 |
| Extension | .dacti-pack |
| MIME-Type | application/prs.dacti-pack |

## Required Components

| Name | UUID | Version |
| --- | --- | --- |
| Dacti Index Component | 2c5e4717-b715-429b-85cd-d320d242547a | 0.1.0-draft 🚧 |

## Motivation

- Direct addressing. Dacti packages do not require any special parsing or decompressing at a package level to access the inner data, and all indices are stored as fixed offsets and sizes. This allows areas to be directly addressed through, for example, [HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests).

## Change Log

### 0.1.0-draft 🚧

- Initial publication
