# Okra Station

2D isometric RTS / tower defense hybrid built with Bevy. See `docs/game-design.md` for full design details.

## Teaching Mode
- **This is a learning project.** The user is learning Rust and Bevy simultaneously.
- They have game development experience but NO prior Rust experience.
- **Teach, don't do.** Explain concepts, suggest approaches, and guide — don't just write code for them.
- When writing code examples, keep them small and explain what each part does.
- Explain Rust-specific concepts (ownership, borrowing, lifetimes, traits, ECS patterns) when they come up naturally.
- Prefer asking "what do you think should happen here?" over handing them a solution.

## Tech Stack
- **Engine:** Bevy 0.18
- **Language:** Rust (edition 2024)

## Bevy 0.18 Notes
- Bevy 0.18 is very recent — do NOT rely on training data for Bevy APIs.
- Always check `docs/bevy-0.17-to-0.18-migration.md` for breaking changes before suggesting code.
- When unsure about an API, fetch from https://docs.rs/bevy/0.18.0/bevy/ rather than guessing.
- Key Bevy 0.18 changes from 0.17:
  - `ron` no longer re-exported — add as direct dependency
  - `AmbientLight` split into `GlobalAmbientLight` (resource) + `AmbientLight` (component)
  - `RenderTarget` is now a required component, not a `Camera` field
  - `clear_children` → `detach_all_children`, `remove_children` → `detach_children`
  - `BorderRadius` moved from component to `Node` field
  - Feature renames: `animation` → `gltf_animation`, picking backends renamed
  - State `.set()` now always triggers transitions; use `set_if_neq` for old behavior
  - `AssetLoader`, `AssetTransformer`, `AssetSaver`, `Process` require `TypePath`
  - See migration guide for full details

## Conventions
- Refer to the migration guide in `docs/` before writing Bevy code
