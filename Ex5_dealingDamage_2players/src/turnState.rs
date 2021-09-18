use specs::Entity;
use std::ptr::null;
use crate::components::InputComponent;
use bracket_terminal::prelude::{GameState, BTerm};

#[derive(Clone)]
pub struct TurnState {
    pub whose_turn: usize,
    pub players: Box<Vec<Entity>>,
    pub polls: Box<Vec<PollInput>>,
}

pub type PollInput = fn(ctx:&BTerm) ->  Option<InputComponent> ;

impl TurnState {

    pub fn next_turn(&mut self) {
        if self.whose_turn ==  self.players.len() - 1  {
            self.whose_turn = 0
        } else {
            self.whose_turn += 1
        }
    }

    pub fn new() ->  TurnState  {
        TurnState { whose_turn: 0, players: Box::new(Vec::new()),polls: Box::new(Vec::new()) }
    }

    pub fn add_player(&mut self, en: Entity, input_polling:PollInput) {
        self.players.push(en);
        self.polls.push(input_polling);
    }

    pub fn current_turn(&self) -> Entity {
        self.players[self.whose_turn]
    }

    pub fn poll_input(&self,ctx:&BTerm) ->  Option<InputComponent> {
        self.polls[self.whose_turn](ctx)
    }

    pub fn remove_current_player(&mut self){
        self.players.remove(self.whose_turn.clone());
        self.polls.remove(self.whose_turn.clone());
        if self.whose_turn == self.players.len() - 1 {
            self.whose_turn = 0
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::turnState::TurnState;
    use specs::Entity;
    use specs::world::Generation;

    #[test]
    fn fov_dupes() {
        let mut state = TurnState::new();

        //state.add_player(Entity::new(0, Generation::new(0)), ());





        //assert_eq!(state.is_player_turn(), true);


    }
}