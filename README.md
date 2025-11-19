
# ezcfg
A simple library for simple configuration.

## Use
1. Add `ezcfg` to your Rust project:
```
cargo add ezcfg
```
2. Use the `cfg` macro to define a struct with your configurable values. 
The types you use must implement `FromStr` and `Display`.
```rust
use ezcfg::*;
use semver::Version;

cfg!{
    MyConfig ["~/.path/to.cfg"]
        version: Version,
        description: String,
        some_value: u8
}
```
3. The struct created by the macro is public within your crate:
```rust
let default = MyConfig {
    version: Version::new(0, 0, 1),
    description: String::from("This is an example config file."),
    some_value: 69u8,
};
```
4. Because we enforce a static path, writing to file is very clean:
```rust
default.write().unwrap();
```
The file at `~/.path/to.cfg` will look like this:
```
version=0.0.1
description=This is an example config file.
some_value=69
```
5. Reading from file into a struct is similarly tidy:
```rust
let mut my_config = MyConfig::read().unwrap();
assert_eq!(my_config.some_value, 69u8);
```
6. As is updating and overwriting values:
```rust
my_config.some_value = 42u8;
my_config.write().unwrap();
```
The file will be changed:
```
version=0.0.1
description=This is an example config file.
some_value=42
```
7. In addition, the path to the configuration file is easily accessible:
```
let path = MyConfig::PATH;
assert_eq!(path, "~/.path/to.cfg");
```

### Troubleshooting
Please note that the `cfg` macro expects a single `ident` token for each type when defining fields.
To use generic types and full module paths as field types, first set up an alias that describes the desired type in a single identifier token:
```rust
// Will fail to compile:
cfg!{
    BadConfig ["~/.badcfg"]
        field: Type<T>,
}

// Do this:
type TypeT = Type<T>;

cfg!{
    OkConfig ["~/.okcfg"]
        field: TypeT,
}
```
Also note that each line follows `field=value`, where `value` can be any type that is representable in string form, as long as the formatted string does not contain the characters `\n` or `=`.

## Development
This crate is mostly complete, and designed to be lightweight and minimal.
New features (e.g., provided methods for `Config`) may be added later if their absence is salient.

If you encounter any bugs, please leave an issue!

