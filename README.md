# Modern History

A real-time territory control simulation built with [Bevy](https://bevyengine.org/) 0.13.

Two factions compete for control of a procedurally divided 256x256 grid map. Armies capture territory through pressure, combat enemies, and receive reinforcements from capitals — all running in real time.

## Features

- 256x256 grid-based map with faction control
- Automated AI with frontline assignment, salient detection, flanking, and retreat logic
- Army spawning, movement, consolidation, and reinforcement from capitals
- Pressure-based territory control with supply lines from capitals
- Real-time combat between opposing armies

## Building

Requires [Rust](https://www.rust-lang.org/tools/install).

```
cargo run
```

## Controls

- **Left click** — Spawn an army for faction -1 (blue)
- **Right click** — Spawn an army for faction +1 (red)

## Architecture

```
src/
  main.rs           — Entry point
  app.rs            — Bevy app setup, system registration
  core/             — Config, game clock (planned)
  map/              — Grid resource, map generation
  army/             — Components, spawning, movement, reinforcement, consolidation
  ai/               — Decision making, tactics, defense, splitting
  simulation/       — Pressure, control, combat, supply, frontline detection
  rendering/        — Map, army, and capital rendering
  city/             — Capitals
```

## Status

Early prototype. See [TODOs.md](TODOs.md) for the development roadmap and known bugs.
