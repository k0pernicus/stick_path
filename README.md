# Stick Path

## Objective

Parse and compute the path(s) from top to bottom based on this kind of diagram:

```
A  B  C
|  |  |
|--|  |
|  |--|
|  |--|
|  |  |
1  2  3
```

The input has to be fill by the user via stdin. 

## Notes

I let the responsability to the developer to parametrize the input / output type of the StickPathSolver structure.  
In the tests and examples, the parameter types are set to: I: String, and O: i32.

## Run the tests && compile the project

```rust
cargo test && cargo build --release
```

## Run the project

```rust
./target/release/stick_path
```
