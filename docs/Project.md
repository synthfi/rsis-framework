# Project
Each RSIS project is serialized to a `rsisproject.toml` file in the root level of the repo.

## Design
This file is designed with the following goals in mind:
- be explicit
- understandable git diffs

## rsisproject.toml
Given an example block name of `fc`.

```toml
name = "fc" # Name of the block/template

[[generate]]
src = "src/fc.toml" # a path to a block definition file
rust = "src/fc.rs"  # enable rust interface geeration
# cpp = ["src/fc.cpp, "src/fc.hpp"] # enable C++ interface generation
# fortran = "src/fc.f90" # enable fortran interface generation

[[dependencies]]
# external block
type="rsis"
url="git"
src = "https://gitlab.mycompany.com/myproject/myblock"

[[dependencies]]
# external rust library to link against
type="rust"
url="git"
src = "..."

[folders]
collections=["collections"]
scenarios=["scenarios"]

[generate]
directory="." # the default directory to place scenario builds
```

## Dependencies
When RSIS downloads external dependencies of type `rsis`, it will grab the following kinds of data and propagate it back for inclusion in the top level project:

- sub-dependencies
- blocks
- collections
- 