# Block
A block represents the smallest unit of execution within RSIS.

Each block is required to have an interface definition file. If a block is called `foo`, then a `foo.toml` file is expected to be found in the `src` directory of the cargo project.

## Elements
Blocks have items that can be accessed publically, referred to as elements. There are four types: inputs, outputs, data, and parameters. Elements map directly to regular rust structs and members in generated code.

Inputs and outputs are also referred to as "ports", and can be connected to support one form of data flow. See [Connection](./Collection.md) for more details.

Data elements are meant to store internal data that might be useful to be easy to access for debugging purposes.

Parameters are tunable items that can be set for different scenarios.

## TOML File
The following example is provided for an example `motor` block.

```toml
type = "block"

# this must exist and be named so
# 
[types.motor]
desc = "motor inputs"
[types.motor.fields]
in = {struct="motor_in"}
out = {struct="motor_out"}
data = {struct="motor_data"}
param = {struct="motor_param"}

[types.motor_in]
desc = "motor model's inputs"
[types.motor_in.fields]
power = {type="f32", dims=[], unit="W", desc="Motor input wattage"}

[types.motor_out]
desc = "motor model's outputs"
[types.motor_out.fields]
speed = {type="f32", dims=[], unit="rad/s", desc="output axle rotational speed"}
torque = {type="f32", dims=[], unit="N/m", desc="output torque"}

[types.motor_data]
desc = "internal motor simulated data"
[types.motor_data.fields]
temp = {type="f32", dims=[], unit="K", desc="internal temperature"}

[types.motor_param]
desc = "motor parameters"
[types.motor_param.fields]
efficiency = {type="f32", dims=[], default = 0.75, desc="how efficient the motor is"}
```

## Save & Load
RSIS supports saving and loading block elements to/from file. This feature is performed using MessagePack integration.

The data is stored in HDF5.

### Saving
Simulation data is saved to file, along with the version of RSIS, and a timestamp. If desired, blocks can be excluded from being saved.

### Loading
RSIS will check that all of the data contained within the save file can be loaded into the simulation without issue, i.e. there are no structural mis-matches, data type mismatches, etc. This defaults to a mode where it checks that the structure of the save file matches that of the simulation exactly. This also supports checking that the save file's structure is a sub-set of the simulation's structure.

After performing the check, RSIS will then load the data into the existing simulation.

Notes
- This feature is primarily supported via autogeneration of supported data types into generated MessagePack code
- If an element has a custom data type, the developer has two options at their disposal
   - provide MessagePack integration for that data type
   - manually modifying the element in the `load` and `save` block functions with data obtained from other sources
