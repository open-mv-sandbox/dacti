# Dacti Object Format

> 🚧 *This is a working document, describing a work-in-progress format. Nothing described in this document should be seen as final. Features described in this document may not be implemented yet, or differ from as described.*

Dacti objects are a flexible self-describing metaverse object format, based on daicon components.

The dacti objects can be used for objects 'at rest'. For example, when stored on a file system, or sent to applications. The object specification can be used to describe the entirety of an object, and to describe a subset of an object, or a replicated mirror of an object for a client to display.

A system supporting dacti objects should support any binary blob of data as an object. Dacti object format allows these binary blobs to describe how they should be interpreted when supported.

| Key | Value |
| --- | --- |
| Name | Dacti Object Format |
| Version | 0.1.0-draft 🚧 |

## Motivation

- Flexibility. Dacti objects enforce very little inherent formats or data. In fact, they're not even required to be 3D mesh objects.
- Extendibility. Dacti objects can be extended for any specific use case, and are designed for this specifically. If you want to make items have special properties in your worlds, dacti objects can bundle this data.

## Components

Dacti objects are extended with purpose-specific data using dacti's 'components'. These components can describe any arbitrary properties.

Often, games or experiences may have their own components to describe metadata specific to that platform. Some components are general enough that they can be useful for multiple different experiences. In both these cases, you can identify these components using daicon's component table.

> 🚧 TODO: Recommended "Metadata Component" specification that gives applications more data for a user to know what an object contains.

## Self-Contained Objects

If stored 'self-contained', the object **MUST** bundle its data for interpretation. When used like this, dacti objects are a superset of "Dacti Packages".

| Key | Value |
| --- | --- |
| Extension | .dacti-obj |
| MIME-Type | application/prs.dacti-obj |

### Required Components

| Name | UUID | Version |
| --- | --- | --- |
| Dacti Index Component | 2c5e4717-b715-429b-85cd-d320d242547a | 0.1.0-draft 🚧 |

## Change Log

### 0.1.0-draft 🚧

- Initial publication
