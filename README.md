# rosrust_dynamic_reconfigure

[![CI Status](https://github.com/ModProg/rosrust_dynamic_reconfigure/actions/workflows/test.yaml/badge.svg)](https://github.com/ModProg/rosrust_dynamic_reconfigure/actions/workflows/test.yaml)
[![Crates.io](https://img.shields.io/crates/v/rosrust_dynamic_reconfigure)](https://crates.io/crates/rosrust_dynamic_reconfigure)
[![Docs.rs](https://img.shields.io/crates/v/template?color=informational&label=docs.rs)](https://docs.rs/rosrust_dynamic_reconfigure)
[![Documentation for `main`](https://img.shields.io/badge/docs-main-informational)](https://modprog.github.io/rosrust_dynamic_reconfigure/rosrust_dynamic_reconfigure/)

This is currently very much WIP and more of an experiment how to best implement
it in rust.

## The `dynamic_reconfigure` API

In the process of developing this crate, I noticed that there wasn't any
conclusive documentation on the API used by `dynamic_reconfigure`. This section
is my attempt at providing that.

The `{namespace}` is the place where the config is "hosted" normally `~`.

Names are expected to match the pattern `[a-zA-Z][a-zA-Z0-9_]*` and descriptions
can be any string not containing quotes, i.e. `"` and `'`[^2]. Though quotes are
probably fine, this might just be a limitation of the original library.

### Endpoints
A Node implementing `dynamic_reconfigure` needs to provide[^3]:

- A latched publisher providing the [`ConfigDescription`](#configdescription-message) at `{namespace}/parameter_descriptions`.
- A latched publisher providing the [`Config`](#config-message) at `{namespace}/parameter_updates`.
- A service with type [`Reconfigure`](#reconfigure-service) at `{namespace}/set_parameters`.

### Properties
Additionally to the publishers/service `dynamic_reconfigure` reads on initialization and writes parameters at
`{namespace}/{parameter_name}`.

### Data Format

#### `ConfigDescription` Message
```
Group[] groups
Config max
Config min
Config dflt
```

##### Values
`max` and `min` describe the ranges for all `int` and `double` values, while
`dflt` contains the default value for all parameters (see [`Config`](#config-message)).

##### Groups
`groups` contains the hierarchical structure of the configuration, with the root
of the configuration being the group with `id=0`, all other groups are
subgroups of the group whose `id` matches their `parent`.

```
string name
string type
ParamDescription[] parameters
int32 parent 
int32 id
```

`type` concerns how the group is displayed in e.g. `rqt_reconfigure`, which
currently supports[^4]:
- empty (`''`) Normal group
- `collapse` Group is collapsible, collapse state is controlled by [`state`](#groups-1)
- `tab` Shows the group as a tab
- `hide` If [`state`](#groups-1) is false hides the group
- `apply` Shows an `Apply` button and only submits input when clicked.

##### ParamDescription
`params` describes the parameters that belong to a group.

```
string name
string type
uint32 level
string description
string edit_method
```

`type` is the data type, either[^1]:
- `str`
- `bool`
- `int`
- `double`

`level` provides a way to optimize partial updates, where the levels of all
modified properties are `|` binary-ored to create a bit mask.

If `edit_method` is non-empty, the field is an enum. The contents are then
expected to be the `repr()` (almost JSON) string of the required constants[^5]:
(one way `repr()` differs from `JSON` is its use of single quotes for strings,
though in the decoding direction `"` should be supported as well, meaning this
is only an issue for parsing as long as no escapes are used.)
```jsonc
{
    "enum_description": "{description}",
    "enum": [ // for every variant
        {
            "name": "{variant_name}",
            "type": "{variant_type}", // Not used by rqt_reconfigure (should be the same as the param)
            "value": "{variant_value}",
            "description": "{variant_description}"
            // .. srcline + srcfile
        }
    ]
}
```

#### `Config` Message
```
BoolParameter[] bools
IntParameter[] ints
StrParameter[] strs
DoubleParameter[] doubles
GroupState[] groups
```

##### Values
`bools`, `ints`, `strs` and `doubles` hold the flattened values for each datatype.

```
string name
{type} value
```

##### Groups
`groups` contain the hierarchical information similar to [`ParamDescription`](#groups).

```
string name
bool state
int32 id
int32 parent
```

`state` controls state of `collapse` and `hide`.

[^1]: https://github.com/ros/dynamic_reconfigure/blob/2654f228adae0848c6e9b70fcf07f890ca6a2841/src/dynamic_reconfigure/parameter_generator.py#L56-L59
[^2]: https://github.com/ros/dynamic_reconfigure/blob/2654f228adae0848c6e9b70fcf07f890ca6a2841/src/dynamic_reconfigure/parameter_generator.py#L64-L74
[^3]: https://github.com/ros/dynamic_reconfigure/blob/2654f228adae0848c6e9b70fcf07f890ca6a2841/src/dynamic_reconfigure/server.py#L77-L83
[^4]: https://github.com/ros-visualization/rqt_reconfigure/blob/988179e349329b5b29f19a891b37d3ac0620ebd2/src/rqt_reconfigure/param_groups.py#L52-L58
[^5]: https://github.com/ros/dynamic_reconfigure/blob/2654f228adae0848c6e9b70fcf07f890ca6a2841/src/dynamic_reconfigure/parameter_generator.py#LL269C21-L269C73
