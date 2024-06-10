use bevy::prelude::*;

use super::components::{MovedChips, PlayerInAction, PlayerIsActive, PreflopPosition, Stack};

pub type PlayerAttributes<'a> = (
    Entity,
    &'a PreflopPosition,
    &'a Stack,
    &'a MovedChips,
    Option<&'a PlayerIsActive>,
    Option<&'a PlayerInAction>,
);

pub struct Betting {
    players: Vec<Player>,
    pot: u32,
    street: u8,
}

impl Betting {
    pub fn new(query_result: &[PlayerAttributes], pot: u32, street: u8) -> Self {
        let mut players: Vec<Player> = query_result.iter().map(Player::from).collect();
        let action_order = if street == 0 {
            PREFLOP_ACTION_ORDER
        } else {
            POSTFLOP_ACTION_ORDER
        };
        players.sort_by_key(|player| {
            let position = player.position;
            let index = action_order.iter().position(|&x| x == position).unwrap();
            index
        });
        Self {
            players,
            pot,
            street,
        }
    }

    pub fn set_actor_inactive(&mut self) {
        let actor = self
            .players
            .iter_mut()
            .find(|player| player.in_action)
            .unwrap();
        actor.is_active = false;
    }

    pub fn max_front(&self) -> u32 {
        self.players
            .iter()
            .map(|player| player.front)
            .max()
            .unwrap()
    }

    pub fn tocall(&self) -> u32 {
        let max_front = self.max_front();
        let actor = self.players.iter().find(|player| player.in_action).unwrap();
        (max_front - actor.front).min(actor.stack)
    }

    pub fn move_options(&self) -> Vec<i32> {
        let max_front = self.max_front();
        let actor = self.players.iter().find(|player| player.in_action).unwrap();
        let tocall: u32 = (max_front - actor.front).min(actor.stack);
        let raise_amount: u32 = if self.street == 0 {
            (self.pot * 2).min(actor.stack)
        } else {
            (self.pot as f32 * 0.5) as u32
        };
        if tocall == 0 {
            vec![0, raise_amount as i32]
        } else {
            vec![-1, tocall as i32, raise_amount as i32]
        }
    }

    pub fn next_actor_id(&self) -> Option<Entity> {
        let n_active_players = self
            .players
            .iter()
            .filter(|player| player.is_active)
            .count();
        if n_active_players < 2 {
            return None;
        }
        let current_actor_index: Option<usize> =
            self.players.iter().position(|player| player.in_action);
        if current_actor_index.is_none() {
            for player in self.players.iter() {
                if player.is_active {
                    return Some(player.entity);
                }
            }
            return None;
        }
        let current_actor_index: usize = current_actor_index.unwrap();
        let mut next_potential_actor: Option<&Player> = None;
        let mut next_potential_actor_index: usize = 0;
        for i in 1..6 {
            let index = (current_actor_index + i) % 6;
            let player = &self.players[index];
            if player.is_active {
                next_potential_actor = Some(player);
                next_potential_actor_index = index;
                break;
            }
        }
        if next_potential_actor.is_none() {
            return None;
        }
        let next_potential_actor: &Player = next_potential_actor.unwrap();

        let max_front = self.max_front();
        let tocall = (max_front - next_potential_actor.front).min(next_potential_actor.stack);

        if tocall == 0 {
            // Determining if potential player has acted
            if max_front > 0 {
                if self.street == 0 && max_front == 2 && next_potential_actor.position == 0 {
                    // bb not acted yet
                    return Some(next_potential_actor.entity);
                } else {
                    return None;
                }
            } else {
                let is_next_potential_player_first_in_action_order = next_potential_actor_index
                    == 0
                    || self.players[..next_potential_actor_index]
                        .iter()
                        .all(|player| !player.is_active);
                if is_next_potential_player_first_in_action_order {
                    return None;
                } else {
                    return Some(next_potential_actor.entity);
                }
            }
        }
        return Some(next_potential_actor.entity);
    }
}

#[derive(Debug)]
struct Player {
    entity: Entity,
    position: u8,
    stack: u32,
    front: u32,
    is_active: bool,
    in_action: bool,
}

impl<'a> From<&PlayerAttributes<'a>> for Player {
    fn from(query: &PlayerAttributes) -> Self {
        let (entity, position, stack, moved_chips, is_active, in_action) = query;
        Self {
            entity: entity.clone(),
            position: position.0,
            stack: stack.0 .0,
            front: moved_chips.0 .0,
            is_active: is_active.is_some(),
            in_action: in_action.is_some(),
        }
    }
}

const PREFLOP_ACTION_ORDER: [u8; 6] = [5, 4, 3, 2, 1, 0];
const POSTFLOP_ACTION_ORDER: [u8; 6] = [1, 0, 5, 4, 3, 2];
