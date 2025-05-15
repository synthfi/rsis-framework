# Scenario
A scenario contains all the information required for autogeneration.

- Instances of blocks
- Links to external libraries and dependencies
- Scheduling of blocks
- Port connections
- Data logging

See [Collection](./Collection.md) for information on block collections.

## Sections
At the root level, the `type = "scenario"` key must exist.

### Scene
This table 
| Key | Type | Description |
| --- | --- | --- |
| name | string | Human readable name |
| desc | string | Description of the scenario |
| engine | string | one of the following: `sim` |

### Block
This table array is used to define blocks and corresponding schedule info.

| Key | Type | Description |
| --- | --- | --- |
| type | string | The name of a custom block, a RSIS utility, or a collection |
| name | string | the name of the block instance. Must be unique |

Example:

```toml
[[schedule]]
type = "sine" # utility
name = "gen1"
freq = 10.0

[[schedule]]
type = "motor" # custom block
name = "MC1"
freq = 10.0
```

### Connection
This table defines connection data. Each key in this table represents groups of connections from an output block to an input block. See [Connection](./Connection.md) for details on what connections are.

Each key must be of the structure `<A>:<B>`, specifying connections from block `A` to block `B`. The value is a list, containing lists of two string elements:
- output block `A`'s port path
- input block `B`'s port path

Example:
```
[connections]
"gen1:gen2" = [
    ["output", "input.amplitude"],
]
```

### Logging
This table defines data logging.


| Key | Type | Description |
| --- | --- | --- |
| rate | numeric | |
| signals | list | Each element is a block element path.

Example:
```toml
[[logging]]
rate = 10.0
signals = [
    "gen2.out.signal"
]
```
