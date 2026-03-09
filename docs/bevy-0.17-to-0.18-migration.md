# Bevy 0.17 to 0.18 Migration Guide

Source: https://bevy.org/learn/migration-guides/0-17-to-0-18/

## Major Breaking Changes

### Entity Events & Components
- `EntityEvent` methods moved to `SetEntityEventTarget` trait for immutability
- `Internal` component removed; engine entities no longer hidden by default
- Entity APIs substantially reworked with new terminology (index vs. row)

### Channel Migration
- `AssetSourceBuilder::with_watcher` now uses `async_channel::Sender` instead of `crossbeam_channel::Sender`

### Feature Renames
- `animation` → `gltf_animation`
- `bevy_sprite_picking_backend` → `sprite_picking`
- `bevy_ui_picking_backend` → `ui_picking`
- `bevy_mesh_picking_backend` → `mesh_picking`

### Rendering & Graphics
- `RenderTarget` moved from `Camera` field to required component
- `dummy_white_gpu_image` removed from pipelines
- `ImageRenderTarget::scale_factor` now `f32` (no `FloatOrd` wrapper needed)
- Draw functions now per-`RenderPhase` instead of per-pass

### System & Scheduler Changes
- `SimpleExecutor` removed; use `SingleThreadedExecutor` or `MultiThreadedExecutor`
- System combinators now treat errors as `false` instead of propagating
- `FunctionSystem` gains new generic `In` parameter
- Schedule error types restructured with new wrapper structs

### Query & Component Changes
- New `ArchetypeQueryData` trait for exact-size iterators
- `QueryData` bounds may need replacement with `ArchetypeQueryData`
- `get_components` methods now return `Result<_, QueryAccessError>`
- `Bundle::component_ids` and `Bundle::get_component_ids` return iterators

### UI & Text
- `BorderRadius` moved from component to `Node` field
- `TextLayoutInfo::section_rects` replaced with `run_geometry`
- `LineHeight` now separate required component
- Non-text areas of `Text` nodes no longer pickable
- `ExtractedUiNode::stack_index` → `z_order` (now `f32`)

### Animation
- `AnimationTarget` split: `id` → `AnimationTargetId`, `player` → `AnimatedBy`
- `AnimationEventTrigger::animation_player` → `target`

### Assets & Loading
- `AssetServer::new` requires `Arc<AssetSources>`
- `AssetProcessor::new` returns tuple with `Arc<AssetSources>`
- `LoadContext::path` returns `AssetPath` (not `Path`)
- `ron` no longer re-exported; add as direct dependency
- `AssetLoader`, `AssetTransformer`, `AssetSaver`, `Process` require `TypePath`

### Material & Rendering
- `MaterialPlugin` fields replaced with `Material` methods: `enable_prepass()`, `enable_shadows()`
- `RenderPipelineDescriptor` uses new `BindGroupLayoutDescriptor`
- `BindGroupLayout` labels now required

### Light & Atmosphere
- `AmbientLight` split: resource `GlobalAmbientLight` + component `AmbientLight`
- `Atmosphere` restructured around `ScatteringMedium` handle

### Mesh Operations
- `Mesh::try_*` functions required for `RenderAssetUsages::RENDER_WORLD`-only meshes
- `Image::reinterpret_size` and `reinterpret_stacked_2d_as_array` return `Result`

### Input
- Input sources in `bevy_input` now require explicit features (mouse, keyboard, gamepad, touch, gestures)

### Entity Hierarchy
Methods renamed for clarity (operations don't despawn entities):
- `clear_children` → `detach_all_children`
- `remove_children` → `detach_children`
- `clear_related` → `detach_all_related`

### State Management
- Setting state now triggers transitions even if identical; use `set_if_neq` for previous behavior

### Other Changes
- `#[reflect(...)]` now supports only parentheses (not braces/brackets)
- `ThinSlicePtr::get()` → `ThinSlicePtr::get_unchecked()`
- `dangling_with_align()` removed; use `NonNull::without_provenance()`
- `Gizmos::cuboid` → `Gizmos::cube`
- `HashMap::get_many_*` → `HashMap::get_disjoint_*`
- `BorderRect` uses `min_inset`/`max_inset` `Vec2` fields
- glTF coordinate conversion now via `GltfConvertCoordinates` struct with separate options
