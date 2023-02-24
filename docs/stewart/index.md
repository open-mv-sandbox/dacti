 # Stewart

> 🚧 This will become the stewart book, currently it just contains quick design notes to be worked out into full documentation.

## Unsorted notes

- Actors receive their own addresses, and can send those addresses to other places. This means that a single actor can be used for a 'public interface' of another actor, as the base actor doesn't have to expose its own address.
- Actors are recursive, they can start and own other actors. Through this, an actor can manage a group of child actors that perform various sub-tasks.
- Context-specific actions are expoed through the starting mechanism. For example, a starting mechanism may expose adding hierarchical child actors. This is intentionally not part of the main actor interface, as these assumptions are not always valid. For example, actors that manage this hierarchy cannot use their own systems that easily.
