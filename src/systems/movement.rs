#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

// system to receive WantToMove messages and apply the movement
// handle all proposed movement by iterate through all entities with a WantsToMove component
#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);
        //  Check if entity is a player, then also update the camera
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination)
        }
    }
    commands.remove(*entity);
}
