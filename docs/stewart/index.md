 # Stewart

> 🚧 This will become the stewart book, currently it just contains quick design notes to be worked out into full documentation.

## Actors

### Addressable

Actors receive their own addresses, and can send those addresses to other places. This means that a single actor can be used for a 'public interface' of another actor, as the base actor doesn't have to expose its own address.

### Recursive

Actors are recursive, they can start and own other actors. Through this, an actor can manage a group of child actors that perform various sub-tasks.
