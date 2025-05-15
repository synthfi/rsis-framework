# Connection
Connections are the primary method provided for passing data between blocks. For asynchronous communication, please see INSERT HERE.

Input and output elements are the only elements that can be connected, and follow several rules:
- Inputs can only be connected to one output
- Outputs can be connected to any number of inputs
- When connected, inputs and outputs cannot be modified after the simulation has passed initialization
- The data types of the output and input elements must be compatible. See the table below:

| Output Type | Allowable Input Types |
| --- | --- |
| i8  | i8, i16, i32, i64 |
| i16 | i16, i32, i64 |
| i32 | i32, i64 |
| i64 | i64 |
| u8  | u8, u16, u32, u64 | 
| u16 | u16, u32, u64 | 
| u32 | u32, u64 | 
| u64 | u64 | 
| f32 | float, double |
| f64 | double |
| bool | bool |
| char | char |

- `String` is not allowed.
- TODO: isize & usize

## Implementation
Connections are implemented at the autogeneration stage. If a connection does not cross between threads, this results in lines of code that copy block outputs to block inputs before a block's run function is executed.

If a connection crosses threads, a more complex set of logic is generated.

## Custom Connections

