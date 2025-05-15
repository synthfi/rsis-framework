# Autogeneration
RSIS makes heavy usage of autogeneration in multiple facets of the toolkit.

## Block
Autogeneration from block interface files is used to generate source code that would be tedious and error-prone to do otherwise. Nominally, this is used to generate rust-code. C++ and Fortran wrappers can additionally be generated to provide integration with libraries written in those languages.

## Scenario
Scenarios are generated into standalone builds/folders. The structure of the scenario is consequently "baked in", and can't be changed without re-generation. However, it is still possible to change parameters as parameters are loaded separately.

