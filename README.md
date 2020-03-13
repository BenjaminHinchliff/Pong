# Pong

## How to run

To run the game, use

```
cargo run
```

on Windows and Linux, and

```
cargo run --no-default-features --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --no-default-features --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.
