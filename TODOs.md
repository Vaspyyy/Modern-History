# Modern History — Development TODOs

> **Bevy version:** 0.13
> **Grid:** 256×256 (65,536 cells), hardcoded in `src/map/mod.rs:10`
> **Last audited:** all source files read and cross-referenced against this document

---

## Dependency Graph

```
Phase 0 ── all independent, do immediately
  │
  ▼
1.1 GameConfig ──────────────────────────── unblocks everything
  │
  ├─▶ 1.2 Game clock ────────────────────── unblocks AI strategy, UI, timed systems
  │
  ├─▶ 1.3 System ordering ───────────────── correct simulation pipeline
  │
  ├─▶ 1.4 Country data model ──┬─────────── unblocks map gen, army scaling, cities, visuals
  │                            │
  │                            ▼
  │               1.5 Map generation ──┬──── needs config + country model
  │                                     │
  │                    ┌────────────────┼────────────────┐
  │                    ▼                ▼                 ▼
  │            2.1 Army scaling   2.2 Cities      2.3 Terrain
  │            (needs 1.4+1.5)   (needs 1.5)     (needs 1.1)
  │                                                  │
  │                              ┌───────────────────┘
  │                              ▼
  │                     3.1 Control diffusion (needs terrain)
  │
  ├─▶ 3.2 Camera controls ────── independent, useful for testing
  ├─▶ 3.3 Chunk rendering ────── independent, perf-critical
  ├─▶ 3.4 Game UI ────────────── needs game clock + country data
  │
  ├─▶ 4.1 Strategic AI ───────── needs game clock + cities
  ├─▶ 4.2 Win/lose conditions ─── independent
  │
  ▼
5.1 Army visuals ── 5.2 Save/load
```

---

## Known Bugs

### ~~KB-1. Supply/combat strength floor mismatch~~ :white_check_mark:
Fixed in TODO 0.6: combat despawn threshold set to `min_army_strength * 0.5`, supply floor uses shared `min_army_strength` from `GameConfig`.

### ~~KB-2. Fragile thread-local frontline cache~~ :white_check_mark:
Fixed in TODO 0.6: replaced `thread_local!` with `CachedFrontline` Bevy `Resource`, written by `assign_new_orders` and read by `assign_orders_timed` and `assign_flanking_orders`.

### ~~KB-3. No system ordering guarantees~~ :white_check_mark:
Fixed in TODO 0.6: added `.chain()` groups for snapshot → consolidate → simulation pipeline → AI pipeline → movement.

### KB-4. `faction as f32` cast in pressure system
- `src/simulation/pressure.rs:35` — `army.faction as f32` works because faction is `-1` or `+1`
- After TODO 1.4 migrates `Army.faction` to `usize`, this will silently produce wrong values (casting `0usize` to `0.0f32` instead of `-1.0`)
- **Fix:** store a `faction_sign: f32` field or compute it via a lookup when the data model changes

---

## Phase 0: Scaffolding & Quick Wins

> No dependencies. All can be done immediately and independently.

### ~~0.1 Bootstrap `core` module~~ :white_check_mark:
`src/core/mod.rs` now declares `pub mod config;` and `pub mod time;`. `mod core;` added to `main.rs`.

### ~~0.2 Register all stub modules~~ :white_check_mark:
All 8 stubs now declared in their parent `mod.rs`:

| Stub file | Parent `mod.rs` | Done |
|---|---|---|
| `src/map/terrain.rs` | `src/map/mod.rs` | :white_check_mark: |
| `src/map/chunk.rs` | `src/map/mod.rs` | :white_check_mark: |
| `src/city/city.rs` | `src/city/mod.rs` | :white_check_mark: |
| `src/ai/strategy.rs` | `src/ai/mod.rs` | :white_check_mark: |
| `src/simulation/diffusion.rs` | `src/simulation/mod.rs` | :white_check_mark: |
| `src/rendering/ui.rs` | `src/rendering/mod.rs` | :white_check_mark: |
| `src/core/config.rs` | `src/core/mod.rs` | :white_check_mark: |
| `src/core/time.rs` | `src/core/mod.rs` | :white_check_mark: |

### ~~0.3 Remove unused `avian2d` dependency~~ :white_check_mark:
Removed `avian2d = "0.6"` from `Cargo.toml`. Verified build succeeds.

### 0.4 Color armies by faction
Currently all armies render as `Color::BLACK` (`src/rendering/army_render.rs:20`).

- Change `color: Color::BLACK` to faction-based: faction `-1` → `Color::rgb(1.0, 0.2, 0.2)`, faction `1` → `Color::rgb(0.2, 0.4, 1.0)` (matching the grid colors in `map_render.rs`)
- This is a one-line change now; a proper `Country.color` lookup comes with TODO 1.4

### ~~0.5 Logging and diagnostics~~ :white_check_mark:
All `println!` calls replaced with structured `tracing` macros (`info!`, `debug!`).
Every subsequent feature will be easier to debug with proper logging instead of `println!`.

- Replace all `println!` with structured logging:
  - `src/app.rs:93` — startup message → `info!("Modern History simulation starting...")`
  - `src/map/mod.rs:39` — grid sample values → `debug!("Sample values: left={}, right={}")`
  - `src/army/spawn.rs:30` — initial army count → `info!("Spawned {} armies per faction", count)`
  - `src/army/spawn.rs:67-70` — click spawn → `debug!("Spawned army at ({:.1}, {:.1}) faction={}")`
  - `src/rendering/capital_render.rs:36` — capitals spawned → `debug!("Capitals spawned")`
  - `src/rendering/map_render.rs:42` — grid visuals spawned → `debug!("Grid visuals spawned")`
- Use `tracing` crate (`info!`, `debug!`, `trace!`) — already re-exported by Bevy
- Use `trace!` for per-frame simulation data (army counts, control values)
- Use `debug!` for AI decisions, reinforcement events, combat outcomes
- Use `info!` for game state changes (startup, victory, defeat)
- Optional: add `--verbose` CLI flag to control log level

### ~~0.6 Fix known bugs~~ :white_check_mark:
Fixed all three known bugs (KB-1, KB-2, KB-3). See commit `4646318`.
Quick fixes that don't require architectural changes:

- **KB-1 (strength floor):** change `combat.rs:29` threshold from `100.0` to `50.0`, or change `supply.rs:28` floor from `100.0` to `150.0` — either way ensures supply can't heal an army into the kill zone
- **KB-2 (frontline cache):** add `CachedFrontline` resource, insert from `assign_new_orders`, read from `assign_orders_timed` and `assign_flanking_orders`
- **KB-3 (system ordering):** add `.chain()` groups in `app.rs` for the critical simulation pipeline (pressure → control → visuals)

---

## Phase 1: Foundation

### ~~1.1 GameConfig system (`src/core/config.rs`)~~ :white_check_mark:
All 30+ scattered constants consolidated into `GameConfig` resource with `Default` impl.
All systems updated to read from `Res<GameConfig>`. Capital positions and initial army
count/positions remain hardcoded (to be derived from `Country` in TODO 1.4).

**Depends on:** Phase 0 (core module bootstrapped)
**Unblocks:** almost everything — terrain, army scaling, reinforcement, AI params, grid size

**Current state:** `src/core/config.rs` is an empty stub. 30+ constants are scattered across 10+ files.

**Constant audit table (exhaustive):**

| Constant | Value | File:Line | Proposed `GameConfig` field |
|---|---|---|---|
| Grid size | 256×256 | `map/mod.rs:10` | `grid_width`, `grid_height` |
| `cell_size` | 3.0 | `pressure.rs:6`, `frontline.rs:6`, `defense.rs:9`, `map_render.rs:11`, `decision.rs:147` | `cell_size` |
| `COMBAT_RADIUS` | 40.0 | `combat.rs:5`, `decision.rs:8` | `combat_radius` |
| `SUPPLY_RANGE` | 200.0 | `supply.rs:22`, `decision.rs:13` | `supply_range` |
| Control speed | 0.0001 | `control.rs:6` | `control_speed` |
| `damage_multiplier` | 0.0005 | `combat.rs:6` | `damage_multiplier` |
| Supply heal rate | 2.0 | `supply.rs:23` | `supply_heal_rate` |
| Supply attrition | 1.0 | `supply.rs:24` | `supply_attrition_rate` |
| `MAX_ARMIES_PER_FACTION` | 15 | `reinforcement.rs:8` | `max_armies_per_faction` |
| `REINFORCE_STRENGTH` | 3000.0 | `reinforcement.rs:9` | `reinforce_strength` |
| `REINFORCE_SPEED` | 8.0 | `reinforcement.rs:10` | `reinforce_speed` |
| `ARMY_SPACING` | 20.0 | `reinforcement.rs:11` | `army_spacing` |
| `STAGGER_INTERVAL` | 2.5 | `reinforcement.rs:12` | `stagger_interval` |
| Capital positions | (-300,0)/(300,0) | `capital_render.rs:17,28`, `reinforcement.rs:14-20` | removed (derived from Country) |
| Initial army count | 5 | `spawn.rs:10` | removed (derived from Country) |
| Initial army strength | 5000.0 | `spawn.rs:17` | `initial_army_strength` |
| Initial army speed | 8.0 | `spawn.rs:19` | `army_speed` |
| Initial army positions | ±250 | `spawn.rs:16,22` | removed (derived from Country) |
| `MERGE_RADIUS` | 10.0 | `consolidation.rs:4` | `merge_radius` |
| `MAX_ARMY_STRENGTH` | 20000.0 | `consolidation.rs:5` | `max_army_strength` |
| `MIN_ARMY_STRENGTH` | 100.0 | `combat.rs:29`, `supply.rs:28` | `min_army_strength` |
| `ARRIVAL_THRESHOLD` | 5.0 | `movement.rs:5` | `arrival_threshold` |
| `SNAPSHOT_INTERVAL` | 1.0s | `grid_history.rs:4` | `snapshot_interval` |
| AI timer | 1.0s | `app.rs:41` | `ai_order_interval` |
| Reinforce timer | 10.0s | `app.rs:42-43` | `reinforce_interval` |
| Split timer | 5.0s | `app.rs:46` | `split_interval` |
| Flank timer | 1.0s | `app.rs:47` | `flank_interval` |
| `STRENGTH_CHECK_RADIUS` | 80.0 | `decision.rs:9` | `strength_check_radius` |
| `RETREAT_STRENGTH` | 500.0 | `decision.rs:10` | `retreat_strength` |
| `RECOVER_STRENGTH` | 1500.0 | `decision.rs:11` | `recover_strength` |
| `MIN_CONSOLIDATE_GROUP` | 4000.0 | `decision.rs:12` | `min_consolidate_group` |
| `NUM_SECTORS` | 5 | `decision.rs:14` | `num_sectors` |
| `DEFEND_RADIUS` | 80.0 | `defense.rs:7` | `defend_radius` |
| `MIN_DEFENDER_STRENGTH` | 1000.0 | `defense.rs:8` | `min_defender_strength` |
| `SPLIT_THRESHOLD` | 10000.0 | `splitting.rs:7` | `split_threshold` |
| `SPLIT_RATIO` | 0.4 | `splitting.rs:8` | `split_ratio` |
| Salient/flank params | various | `tactics.rs:7-12` | grouped in `FlankConfig` |

**Implementation:**

- Create `GameConfig` struct with all fields above
- Insert as a Bevy resource at startup (in `app.rs`)
- Update all systems to read from `Res<GameConfig>` instead of local constants
- Remove all local `const` declarations
- Supersedes KB-1 fix from 0.6 — use shared `min_army_strength` everywhere

### 1.2 Game clock (`src/core/time.rs`)

**Depends on:** Phase 0 (core module bootstrapped)
**Unblocks:** AI strategy (war exhaustion), UI (speed controls), all timed systems

**Current state:** `src/core/time.rs` is an empty stub. All systems use Bevy's raw `time.delta()`.

**Implementation:**

- `GameSpeed` enum: `Paused`, `Normal(1x)`, `Fast(2x)`, `Faster(4x)`
- `GameClock` resource: `elapsed_days: f32`, `speed: GameSpeed`, `total_paused_time: f32`
- `game_delta()` method: returns `real_delta * speed_multiplier`, or `0.0` if paused
- Replace all `time.delta_seconds()` calls with `game_clock.game_delta(&time)` in:
  - `movement.rs:27`
  - `reinforcement.rs:29` (timer tick)
  - `grid_history.rs:30` (timer tick)
  - `splitting.rs:69` (timer tick)
  - `tactics.rs:211` (timer tick)
  - `decision.rs:432` (timer tick)
  - `control.rs:6` (pressure application)
- Keyboard shortcuts: `Space` = toggle pause, `+`/`-` = speed up/down

### ~~1.3 System ordering~~ :white_check_mark:
Replaced single `.chain()` with explicit `SystemSet` labels: `SimulationSet`, `AISet`,
`MovementSet` — each with internal chaining. Visuals (`update_grid_visuals`) run after
`SimulationSet` but before `AISet`. `spawn_army_on_click` runs independently.

**Depends on:** 1.1 (so GameConfig is available during the restructure)
**Unblocks:** correct simulation behavior for all subsequent features

**Current state:** All `Update` systems in `src/app.rs:72-87` run with no ordering guarantees. The Phase 0 fix (0.6) applied a basic `.chain()` but a proper system architecture is needed.

**Implementation:**

- Define simulation pipeline as chained systems:
  ```
  Simulation: snapshot_control → apply_pressure → apply_supply → apply_combat → update_control
  ```
- AI pipeline runs after simulation:
  ```
  AI: assign_orders → assign_flanking → defend_breakthroughs → ai_split_armies → move_armies
  ```
- Reinforcement runs last (spawns new entities for next frame)
- Visuals run in `PostUpdate` (already partially the case)
- Timer-based systems use run conditions instead of manual timer checks where possible

### 1.4 Faction/Country data model (`src/core/faction.rs` — new file)

**Depends on:** 1.1 (config for country parameters)
**Unblocks:** map generation, army scaling, cities, visuals, AI faction logic

**Current state:** `Army.faction` is `i32` (`-1` or `+1`). `Capital.faction` is `i32`. Faction is used as a multiplier in `pressure.rs:35` (`army.faction as f32`).

**Implementation:**

- `Country` struct: `id: usize`, `name: String`, `color: Color`, `capital_pos: Vec2`, `territory_cells: Vec<(usize, usize)>`, `starting_army_count: usize`, `reinforcement_rate: f32`, `richness: f32`
- `Countries` resource: `Vec<Country>` — the full list of selectable countries
- `SelectedFactions` resource: `(usize, usize)` — which two countries are fighting
- `FactionSign(f32)` helper: wraps the sign convention for pressure/combat (fixes KB-4)
- Update `Army.faction` from `i32` to `usize` (country ID)
- Update `Capital.faction` from `i32` to `usize`
- Update all systems that reference `faction: i32`:
  - `src/ai/decision.rs` — sector computation, force ratios, retreat logic
  - `src/ai/tactics.rs` — salient detection, flank assignment
  - `src/ai/defense.rs` — threat detection, breakthrough defense
  - `src/ai/splitting.rs` — faction count tracking
  - `src/simulation/combat.rs` — enemy detection
  - `src/simulation/supply.rs` — friendly capital lookup
  - `src/simulation/pressure.rs` — influence direction (critical: fix `as f32` cast, KB-4)
  - `src/rendering/army_render.rs` — visual color (use `Country.color`)
  - `src/rendering/capital_render.rs` — visual color (use `Country.color`)
  - `src/army/spawn.rs` — faction assignment
  - `src/army/consolidation.rs` — friendly/enemy detection
  - `src/army/reinforcement.rs` — faction count, capital lookup

### 1.5 Map generation (`src/map/generation.rs` — new file)

**Depends on:** 1.1 (config for grid size), 1.4 (Countries resource)
**Unblocks:** army scaling, cities, terrain placement

**Current state:** `src/map/mod.rs:14-39` — blank 256×256 grid with a hard-coded vertical line. No `generation.rs` exists.

**Implementation:**

- Generate N territories using Voronoi regions from random seed points
- Organic borders via distance-based falloff between territory centers
- Variable territory sizes (some countries bigger, some smaller)
- Each territory gets a capital position (centroid or strategic point)
- Territory "richness" value (affects army scaling in Phase 2) — based on size, shape compactness
- Grid size from `GameConfig` (support 512×512+)
- Store generated territories into the `Countries` resource
- Seed-based generation for reproducibility (store seed in `GameConfig`)
- Replace the current `setup_grid` function in `map/mod.rs`

---

## Phase 2: Economy & Scaling

### 2.1 Terrain system (`src/map/terrain.rs`)

**Depends on:** 1.1 (config for terrain params)
**Unblocks:** control diffusion (terrain-based modifiers)

**Current state:** `src/map/terrain.rs` is an empty stub. `Cell` struct has `control` and `pressure` fields only.

**Implementation:**

- Terrain types: Plains (normal), Forest (speed -25%, defense +20%), Mountain (impassable), Hill (speed -15%, defense +30%), River (speed -50%), Desert (supply drain +50%)
- `TerrainType` enum with speed/modifier/defense fields
- Add `terrain: TerrainType` field to `Cell` struct (`src/map/grid.rs:4-7`)
- Generate terrain using noise (Perlin/Simplex) — mountains along borders, rivers between territories
- `map_render.rs`: render terrain with tinted colors (green forests, brown mountains, blue rivers)
- `movement.rs:27`: check terrain at army position, apply speed modifier
- `combat.rs`: terrain defense bonus reduces damage taken
- `supply.rs`: desert terrain increases supply drain
- Terrain config (noise seed, type thresholds) in `GameConfig`
- Register `pub mod terrain;` in `src/map/mod.rs` (currently missing — see TODO 0.2)

### 2.2 Army scaling tied to country size/richness

**Depends on:** 1.4 (Country data), 1.5 (map generation)

**Files:** `src/army/spawn.rs`, `src/army/reinforcement.rs`

**Current state:** Hardcoded `5` initial armies, `3000.0` reinforce strength, `15` max armies per faction, capitals at fixed positions.

**Implementation:**

- `starting_army_count = base_count + (territory_cells / cells_per_army)` — formula in `GameConfig`
- `reinforcement_rate` proportional to controlled territory cell count (loses territory = slower reinforcement)
- Army strength per spawn scaled by country `richness`
- Reinforcement cap (`MAX_ARMIES_PER_FACTION`) proportional to territory size
- Spawning positions distributed along the frontline or near cities, not just at capital
- Remove `capital_position()` function from `reinforcement.rs` (use `Country.capital_pos`)

### 2.3 Non-capital cities (`src/city/city.rs`)

**Depends on:** 1.5 (map generation for city placement)

**Current state:** `src/city/city.rs` is an empty stub. Only 2 hardcoded `Capital` entities exist (`src/city/capital.rs`, spawned in `src/rendering/capital_render.rs`).

**Implementation:**

- `City` component: `id: usize`, `faction: usize`, `is_capital: bool`, `supply_range: f32`, `position: Vec2`
- Generate cities during map gen based on territory size (more territory = more cities)
- Cities provide: extended supply range, reinforcement spawn points, retreat destinations
- City capture: when enemy control around a city exceeds threshold, city changes faction
- Update `supply.rs` to check proximity to any friendly city (not just capitals)
- Update `reinforcement.rs` to spawn at cities (not just capitals)
- AI should prioritize defending/attacking cities, not just raw frontline
- Register `pub mod city;` in `src/city/mod.rs` (currently missing — see TODO 0.2)

---

## Phase 3: Rendering & Performance

### 3.1 Camera controls

**Depends on:** 1.1 (config for pan/zoom speeds)
**Why here:** Independent of other Phase 2/3 work, but makes testing and playing much easier.

**Current state:** Single `Camera2dBundle::default()` in `src/app.rs:94` with no movement. The map is 768×768 world units (256×3.0) but there's no way to navigate it.

**Implementation:**

- WASD or arrow-key camera panning
- Scroll-wheel zoom (clamp between min/max zoom in `GameConfig`)
- Edge-of-screen panning (optional)
- Camera bounds: clamp position so it can't leave the map
- Smooth movement with configurable pan speed and zoom speed

### 3.2 Chunk-based grid rendering (`src/map/chunk.rs`)

**Depends on:** 1.1 (config for chunk size)

**Current state:** `spawn_grid_visuals` creates 65,536 individual `SpriteBundle` entities. `update_grid_visuals` iterates ALL of them every frame regardless of whether they changed.

**Performance problem:** 256×256 = 65,536 sprite entities + 65,536 sprite updates per frame. A 512×512 grid would be 262,144 entities — likely unplayable.

**Implementation:**

- Divide grid into N×N chunks (e.g., 16×16 cells per chunk = 256 chunks for 256×256)
- Each chunk is a single entity with a dynamically-generated texture
- Track dirty chunks: only regenerate texture for chunks whose cells changed
- Target: reduce per-frame work from 65K entity updates to ~256 chunk updates (worst case)
- Future: GPU compute shader for control simulation, removing CPU-side grid iteration entirely
- Register `pub mod chunk;` in `src/map/mod.rs` (currently missing — see TODO 0.2)

### 3.3 Game UI (`src/rendering/ui.rs`)

**Depends on:** 1.2 (game clock for speed display), 1.4 (country data for faction info)

**Current state:** `src/rendering/ui.rs` is an empty stub. The only interaction is pressing `1`/`2` and clicking.

**Implementation:**

- **Country selection screen**: after map generation, show N countries as clickable territories; player picks 2 to fight
- **HUD**: army count per faction, territory %, total army strength, game time / day counter
- **Game speed controls**: pause, 1x, 2x, 4x buttons (wired to `GameClock`)
- **Info panel**: hover over army to see strength/faction/orders
- Use Bevy UI (`NodeBundle`, `TextBundle`, `ButtonBundle`) — note: Bevy 0.13 uses `NodeBundle` not `UiCameraBundle`
- Register `pub mod ui;` in `src/rendering/mod.rs` (currently missing — see TODO 0.2)

---

## Phase 4: Gameplay Depth

### 4.1 Control diffusion (`src/simulation/diffusion.rs`)

**Depends on:** 2.1 (terrain for diffusion modifiers)

**Current state:** `src/simulation/diffusion.rs` is an empty stub. Territory only changes from army pressure — borders are sharp and defined entirely by army positions.

**Implementation:**

- Each frame, each cell's control shifts slightly toward the average of its neighbors
- Diffusion rate much slower than army pressure (background spread)
- Creates more organic, flowing borders instead of sharp pressure-defined lines
- Diffusion rate affected by terrain: mountains block diffusion, rivers slow it
- Add `diffusion_rate: f32` to `GameConfig`
- Register `pub mod diffusion;` in `src/simulation/mod.rs` (currently missing — see TODO 0.2)
- Add diffusion system to simulation pipeline in `app.rs` (after `apply_pressure`, before `update_control`)

### 4.2 Strategic AI (`src/ai/strategy.rs`)

**Depends on:** 1.2 (game clock for war exhaustion timing), 2.3 (cities for strategic targeting)

**Current state:** `src/ai/strategy.rs` is an empty stub. All AI is tactical: sector assignment (`decision.rs`), flanking (`tactics.rs`), retreat (`decision.rs`), breakthrough defense (`defense.rs`), and splitting (`splitting.rs`).

**Implementation:**

- **Stance system**: `FactionStrategy` resource per faction — `Offensive`, `Defensive`, `Balanced`
- Stance affects: army behavior (aggressive push vs. hold line), reinforcement priority, willingness to flank
- **Multi-front priority**: when fighting on multiple fronts (e.g., after city capture creates enclaves), prioritize which front gets reinforcements
- **War exhaustion**: `WarExhaustion` resource tracking cumulative losses; exhausted factions shift to defensive stance
- **Strategic timing**: detect when enemy is weak (low total army strength, many retreating) and switch to offensive
- Register `pub mod strategy;` in `src/ai/mod.rs` (currently missing — see TODO 0.2)

### 4.3 Win/lose conditions

**Depends on:** 1.4 (country data for capital identification)

**Current state:** The simulation runs indefinitely with no victory detection.

**Implementation:**

- Victory condition: control > 80% of total cells, OR enemy capital captured (control near capital > 0.9)
- Defeat condition: own capital captured, OR control drops below 10%
- On victory/defeat: display result screen with stats (duration, casualties, territory gained)
- Option to restart with new map or same map
- Check conditions once per second (not every frame) using `GameClock`

---

## Phase 5: Polish

### 5.1 Army visual improvements
- ~~Color armies by faction~~ (done in TODO 0.4, upgrade to `Country.color` in TODO 1.4)
- Size proportional to strength (already partially done — `army_render.rs:56`)
- Show movement direction indicator (line or arrow from army position toward `ArmyOrder.target`)
- Distinct visual for retreating (faded/pulsing), flanking (angular marker), defending (shield icon) states
- Strength number label above armies (already exists in `army_render.rs:33`)

### 5.2 Save/load system

**Current state:** No game state persistence exists.

**Implementation:**

- Serialize `Grid` cells, all `Army` entities, all `City` components, `GameClock`, `Countries`
- Save to file via `serde` + `ron` (add to `Cargo.toml`)
- Load: reconstruct all entities and resources from save data
- Quick-save / quick-load hotkeys
- Auto-save every N game-days
