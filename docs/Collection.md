# Collection
Groups of blocks can be assembled into a `Collection` for ease of re-use. Functionally, these are very similar to [scenarios](./Scenario.md), with certain sections being restricted from use (logging).

Collections are allowed to contain other collections, with the exception of itself in itself or itself in any sub-collection.

## Block Name Resolution
Paths to blocks within collections are specified with the `/` character in any location where a unique block name is expected.

Given the following example:
- A collection named `Car` contains the block
   - `engine`
- a scenario instantiates the `Car` collection twice:
   - `toyota`
   - `honda`

The toyota's engine would be referred to via `toyota/engine`, and the honda's engine would be referred to via `honda/engine`.
