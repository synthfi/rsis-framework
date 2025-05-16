# Template
A template is a kind of block that is generic, allowing itself to be generated for usage with specific data types. Templates are typically implemented as generic rust structs, and have definitions provided with TOML to allow for data type checking to occur. They can also be implemented via the autogeneration stage.

## Template Block
An example is provided for the delay block, which implements two kinds of generics, type and size.

```toml
[template]
name = "<name>"
desc = "<description>"

# `value` is used for the default when there is no known selection
[generic]
T = { type="datatype", value="f64", desc="Input datatype"}
M = { type="sized", value=[], desc="Input dimensions"}
N = { type="sized", value=1, desc="Number of samples to delay"}

[types.delay]
in = {struct="delay_in"}
out = {struct="delay_out"}
data = {struct="delay_data"}
# no parameters

[types.delay_in]
desc = "delay block inputs"
[types.delay_in.fields]
in = {type="T", dims="M", desc="input signal to be delayed"}

[types.delay_out]
desc = "delay block outputs"
[types.delay_out.fields]
out = {type="T", dims="M", desc="delayed input signal"}

[types.delay_data]
desc = "internal data"
[types.delay_data.fields]
buffer = {type="T", dims=["N"], desc="internal delay buffer"}
```

## Restrictions
Generics can be restricted to specific datatypes.
```toml
[generic]
T = {type="datatype", value="f64", options=["f32", "f64"], desc="signal type"}
```
