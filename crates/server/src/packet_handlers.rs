use base::Position;
use common::Game;
use ecs::{Entity, EntityRef, SysResult};
use protocol::{
    packets::{
        client,
        server::{Animation, Hand},
    },
    ClientPlayPacket,
};

use crate::{NetworkId, Server};

mod movement;

/// Handles a packet received from a client.
pub fn handle_packet(
    game: &mut Game,
    server: &mut Server,
    player: Entity,
    packet: ClientPlayPacket,
) -> SysResult {
    let player = game.ecs.entity(player)?;
    match packet {
        ClientPlayPacket::PlayerPosition(packet) => {
            movement::handle_player_position(player, packet)
        }
        ClientPlayPacket::PlayerPositionAndRotation(packet) => {
            movement::handle_player_position_and_rotation(player, packet)
        }
        ClientPlayPacket::PlayerRotation(packet) => {
            movement::handle_player_rotation(player, packet)
        }
        ClientPlayPacket::PlayerMovement(packet) => {
            movement::handle_player_movement(player, packet)
        }

        ClientPlayPacket::Animation(packet) => handle_animation(server, player, packet),

        ClientPlayPacket::TeleportConfirm(_)
        | ClientPlayPacket::QueryBlockNbt(_)
        | ClientPlayPacket::SetDifficulty(_)
        | ClientPlayPacket::ChatMessage(_)
        | ClientPlayPacket::ClientStatus(_)
        | ClientPlayPacket::ClientSettings(_)
        | ClientPlayPacket::TabComplete(_)
        | ClientPlayPacket::WindowConfirmation(_)
        | ClientPlayPacket::ClickWindowButton(_)
        | ClientPlayPacket::ClickWindow(_)
        | ClientPlayPacket::CloseWindow(_)
        | ClientPlayPacket::PluginMessage(_)
        | ClientPlayPacket::EditBook(_)
        | ClientPlayPacket::QueryEntityNbt(_)
        | ClientPlayPacket::InteractEntity(_)
        | ClientPlayPacket::GenerateStructure(_)
        | ClientPlayPacket::KeepAlive(_)
        | ClientPlayPacket::LockDifficulty(_)
        | ClientPlayPacket::VehicleMove(_)
        | ClientPlayPacket::SteerBoat(_)
        | ClientPlayPacket::PickItem(_)
        | ClientPlayPacket::CraftRecipeRequest(_)
        | ClientPlayPacket::PlayerAbilities(_)
        | ClientPlayPacket::PlayerDigging(_)
        | ClientPlayPacket::EntityAction(_)
        | ClientPlayPacket::SteerVehicle(_)
        | ClientPlayPacket::SetDisplayedRecipe(_)
        | ClientPlayPacket::SetRecipeBookState(_)
        | ClientPlayPacket::NameItem(_)
        | ClientPlayPacket::ResourcePackStatus(_)
        | ClientPlayPacket::AdvancementTab(_)
        | ClientPlayPacket::SelectTrade(_)
        | ClientPlayPacket::SetBeaconEffect(_)
        | ClientPlayPacket::HeldItemChange(_)
        | ClientPlayPacket::UpdateCommandBlock(_)
        | ClientPlayPacket::UpdateCommandBlockMinecart(_)
        | ClientPlayPacket::CreativeInventoryAction(_)
        | ClientPlayPacket::UpdateJigsawBlock(_)
        | ClientPlayPacket::UpdateStructureBlock(_)
        | ClientPlayPacket::UpdateSign(_)
        | ClientPlayPacket::Spectate(_)
        | ClientPlayPacket::PlayerBlockPlacement(_)
        | ClientPlayPacket::UseItem(_) => Ok(()),
    }
}

fn handle_animation(
    server: &mut Server,
    player: EntityRef,
    packet: client::Animation,
) -> SysResult {
    let pos = *player.get::<Position>()?;
    let network_id = *player.get::<NetworkId>()?;

    let animation = match packet.hand {
        Hand::Main => Animation::SwingMainArm,
        Hand::Off => Animation::SwingOffhand,
    };

    server.broadcast_nearby_with(pos, |client| {
        client.send_entity_animation(network_id, animation.clone())
    });
    Ok(())
}
