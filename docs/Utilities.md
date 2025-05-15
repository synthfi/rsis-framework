# Utilities
A list of provided template utilities that are always available:
- [Delay](#delay)
- [Sine](#sine)
- [Switch](#switch)

A list of utilities only available with the Julia wrapper:
- [Unit Conversion](#unit-conversion)


## Delay
Implements a variable signal delay. This block does not use dynamic allocation.

| Generic | Usage | Notes |
| --- | --- | --- |
| T | Signal datatype | Restricted to RSIS types |
| M | Signal dimension | Leave as default `0` for the scalar case |
| N | Number of samples to delay | Must be >= 1 |

## Halt
Halts the simulation when requested.

## Sine
Provides a generated sine wave.

Emulates `amplitude * sine(omega * t + phase) + bias`, but internally tracks an internal phase to handle long periods of simulated time.

| Generic | Usage | Notes |
| --- | --- | --- |
| T | Output datatype | Restricted to `f32`, `f64` |

| Parameter | Datatype | Notes |
| --- | --- | --- |
| amplitude | T | Signal amplitude |
| bias | T | Signal bias |
| phase | T | Offset from clock time |
| omega | T | scaling of time parameter |

## Switch
Passes a selected input to the output. Each input and the output must have the same datatype and size. Internally implemented via autogeneration.

| Generic | Usage | Notes |
| --- | --- | --- |
| T | Signal datatype | Restricted to RSiS types |
| M | Number of inputs | |
| N | Signal dimension | Leave as default `0` for scalars |

## Unit Conversion
Implements unit conversion. Relies on the Julia wrapper. Internally implemented via autogeneration.
