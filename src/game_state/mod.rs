use crate::game_data::{Player, COLS, ROWS};
use ggez::event::KeyCode;
use std::env;

use libloading::{Library, Symbol};

pub const PLAYER_AMOUNT: usize = 2;
/// Function signature for the ai-script
type AIFunc = unsafe fn(*const [[u32; 10]; 24], *const [[i32; 2]; 4], *const [[i32; 2]; 4]) -> u32;

#[cfg(test)]
mod tests;

pub struct Game {
    players: [Player; PLAYER_AMOUNT],
    ai_lib: [Option<Library>; 2],
}

impl Game {
    pub fn new(init_level: usize) -> Game {
        let library: Option<Library>;
        if let Some(lib_path) = env::args().nth(1) {
            if cfg!(windows) && !lib_path.ends_with(".dll") {
                panic!("Must use .dll if running an AI in windows!")
            } else if cfg!(unix) && !lib_path.ends_with(".so") {
                panic!("Must use .so if running an AI in a 'nix-system!")
            }
            if let Ok(lib) = Library::new(lib_path) {
                library = Some(lib);
            } else {
                library = None;
            }
        } else {
            library = None;
        }
        let library2: Option<Library>;
        if let Some(lib_path2) = env::args().nth(2) {
            if cfg!(windows) && !lib_path2.ends_with(".dll") {
                panic!("Must use .dll if running an AI in windows!")
            } else if cfg!(unix) && !lib_path2.ends_with(".so") {
                panic!("Must use .so if running an AI in a 'nix-system!")
            }
            if let Ok(lib2) = Library::new(lib_path2) {
                library2 = Some(lib2);
            } else {
                library2 = None;
            }
        } else {
            library2 = None;
        }

        Game {
            players: [Player::new(init_level), Player::new(init_level)],
            ai_lib: [library2, library],
        }
    }
    /// The game-tick update function
    pub fn update(&mut self) {
        // update game tick for players
        let mut target_mod: i32 = 1; //Pairs, you attack the one next to you
        for p in 0..self.players.len() {
            self.players[p].update();
            //attack handling
            if let Some(attack) = self.players[p].take_outgoing() {
                self.players[(p as i32 + target_mod) as usize].add_incoming(attack);
            }

            target_mod *= -1;
        }
        for i in 0..self.ai_lib.len() {
            if self.ai_lib[i].is_some() {
                let ai_output = self.call_ai_script(i);
                self.parse_ai_output(i, ai_output);
            }
        }
    }
    /// Gets and returns the graphical boardstate of the players
    pub fn get_boards(&self) -> [[[u32; COLS]; ROWS]; PLAYER_AMOUNT] {
        [
            self.players[0].get_board_visual(),
            self.players[1].get_board_visual(),
        ]
    }
    /// Gets and returns the next pieces of the players
    pub fn get_next_pieces(&self) -> [[[u32; 4]; 4]; PLAYER_AMOUNT] {
        let mut next_pieces = [[[0; 4]; 4]; PLAYER_AMOUNT];
        for p in 0..self.players.len() {
            next_pieces[p] = self.players[p].get_next_piece().get_display_shape();
        }
        next_pieces
    }
    /// Gets and returns the saved pieces of the players
    pub fn get_saved_pieces(&self) -> [[[u32; 4]; 4]; PLAYER_AMOUNT] {
        let mut saved_pieces = [[[0; 4]; 4]; PLAYER_AMOUNT];
        for p in 0..self.players.len() {
            if let Some(piece) = self.players[p].get_saved_piece() {
                saved_pieces[p] = piece.get_display_shape();
            }
        }
        saved_pieces
    }
    /// Gets the incoming attacks from players and returns formatted data
    pub fn get_attackbars(&self) -> [u32; PLAYER_AMOUNT] {
        let mut attackbars = [0; PLAYER_AMOUNT];
        for p in 0..self.players.len() {
            for (attack, _) in self.players[p].get_incoming() {
                attackbars[p] += *attack as u32;
            }
        }
        attackbars
    }
    /// Returns formatted data for the ai-script, without block-projection.
    pub fn get_player_data(
        &self,
        index: usize,
    ) -> ([[u32; COLS]; ROWS], [[i32; 2]; 4], [[i32; 2]; 4]) {
        let mut data = ([[0; COLS]; ROWS], [[0; 2]; 4], [[0; 2]; 4]);
        if index < self.players.len() {
            let p = &self.players[index];
            data = (p.get_board(), p.get_current_shape(), p.get_saved_shape());
        }
        data
    }

    pub fn get_scores(&self) -> [u32; PLAYER_AMOUNT] {
        let mut scores = [0; PLAYER_AMOUNT];
        for p in 0..self.players.len() {
            scores[p] = self.players[p].get_score() as u32;
        }
        scores
    }

    pub fn get_losts(&self) -> [bool; PLAYER_AMOUNT] {
        [self.players[0].get_lost(), self.players[1].get_lost()]
    }

    //Only set for 2 players
    pub fn key_down(&mut self, key: KeyCode) {
        if self.ai_lib[0].is_none() {
            match key {
                // P1 controlls
                KeyCode::A => self.players[0].move_current(-1, 0),
                KeyCode::E => self.players[0].rotate_current(true),
                KeyCode::D => self.players[0].move_current(1, 0),
                KeyCode::Q => self.players[0].rotate_current(false),
                KeyCode::S => self.players[0].move_current(0, -1),
                KeyCode::W => self.players[0].drop_current(),
                KeyCode::Space => self.players[0].save_piece(),
                _ => (),
            }
        }
        if self.ai_lib[1].is_none() {
            match key {
                // P2 controlls
                KeyCode::J => self.players[1].move_current(-1, 0),
                KeyCode::O => self.players[1].rotate_current(true),
                KeyCode::L => self.players[1].move_current(1, 0),
                KeyCode::U => self.players[1].rotate_current(false),
                KeyCode::K => self.players[1].move_current(0, -1),
                KeyCode::RShift => self.players[1].save_piece(),
                KeyCode::I => self.players[1].drop_current(),
                _ => (),
            }
        }
    }

    pub fn restart(&mut self, init_level: usize) {
        self.players = [Player::new(init_level), Player::new(init_level)];
    }

    fn call_ai_script(&mut self, player_index: usize) -> u32 {
        let mut output = 0;

        unsafe {
            if let Some(lib) = &self.ai_lib[player_index] {
                let func: Symbol<AIFunc> = lib.get(b"ai").expect("Couldn't find ai function");
                let (board, current_piece, saved_piece) = self.get_player_data(1);
                output = func(&board, &current_piece, &saved_piece);
            }
        }
        output
    }

    fn parse_ai_output(&mut self, player_index: usize, output: u32) {
        match output {
            1 => self.players[player_index].move_current(-1, 0),
            2 => self.players[player_index].move_current(1, 0),
            3 => self.players[player_index].rotate_current(true),
            4 => self.players[player_index].rotate_current(false),
            5 => self.players[player_index].move_current(0, -1),
            6 => self.players[player_index].drop_current(),
            7 => self.players[player_index].save_piece(),
            _ => (),
        }
    }
}
