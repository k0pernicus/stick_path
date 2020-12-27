# Stick Path

## Objective


"Stick Path" begins with a number of vertical lines.  
Between the lines are random horizontal connectors that connect all the lines in a connected diagram, like the one below.

```
A  B  C
|  |  |
|--|  |
|  |--|
|  |--|
|  |  |
1  2  3
```

To play the game, the player selects a line at the top and follows it down.  
When he meets a horizontal connector, he must follow it to move to another vertical line and continue down.  
Repeat this process until you reach the bottom of the diagram.

At the end, list all connected pairs.

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
