use bytemuck::{Pod, Zeroable};
/// Marker component for area effect cloud entities.
///
/// # Example
/// A system that queries for all area effect clouds:
/// ```no_run
/// use quill::{Game, Position, entities::AreaEffectCloud};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &AreaEffectCloud)>() {
///         println!("Found a area effect cloud with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct AreaEffectCloud;

pod_component_impl!(AreaEffectCloud);
