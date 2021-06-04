# Caffeine

Jiggle the mouse to prevent screen lock, idle, etc from triggering.

## Usage

Jiggle the mouse a single time:
```
./caffeine
```

Jiggle the mouse forever, waiting 90 seconds between loops:
```
./caffeine --sleep=90
```

Jiggle the mouse, moving a larger distance with a greater pause between movements (in case of insensitive movement detection):
```
./caffeine --pixels=20 --delay=100
```

## Building

Non-rust dependencies that must be installed. For Ubuntu

```
apt-get install libx11-dev libxtst-dev
```

Can then proceed with

```
cargo build --release
```

## License

This project is dual-licensed under Apache-2.0 and MIT.
