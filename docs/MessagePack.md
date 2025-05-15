# MessagePack
MessagePack is a binary serialization format that is used within RSIS for several purposes:

- Julia integration with scenarios that have been compiled to a machine-specific library
- Integration of C++ and Fortran libraries into Rust when FFI is unsuitable (containers and dynamically allocated datatypes)
- 

Generated MessagePack code is gated behind a feature, allowing for that code to not be compiled into the final library if desired. The rest of this page assumes that MessagePack has been enabled for the build.

## Library Features
All compiled scenario builds will be able to perform the following via a MessagePack interface:
- provide information on the structure of the simulation. E.g. blocks, scheduling info, number of expected threads, thread frequencies, etc.
- provide the interface for all blocks
- get and set block element values

## Low Level Integration
The generated Rust library will contain a set of functions exposed via the C ABI.

