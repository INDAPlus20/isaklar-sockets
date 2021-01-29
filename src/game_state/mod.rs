use crate::game_data::{Color, Piece, Player, COLS, ROWS, SHAPES};
use ggez::{event::KeyCode, graphics::pipe::new};
use rand::distributions::uniform;
use std::env;

use libloading::{Library, Symbol};

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

pub const PLAYER_AMOUNT: usize = 2;
/// Function signature for the ai-script
type AIFunc = unsafe fn(*const [[u32; 10]; 24], *const [[i32; 2]; 4], *const [[i32; 2]; 4]) -> u32;
type Packet = [u8; 2];
#[cfg(test)]
mod tests;

pub struct Game {
    players: [Player; PLAYER_AMOUNT],
    ai_lib: [Option<Library>; 2],
    recieved_moves: mpsc::Receiver<Packet>, // channel for networking
    moves_to_send: mpsc::Sender<Packet>,
}

impl Game {
    pub fn new(init_level: usize) -> Game {
        // let library: Option<Library>;
        // if let Some(lib_path) = env::args().nth(1) {
        //     if cfg!(windows) && !lib_path.ends_with(".dll") {
        //         panic!("Must use .dll if running an AI in windows!")
        //     } else if cfg!(unix) && !lib_path.ends_with(".so") {
        //         panic!("Must use .so if running an AI in a 'nix-system!")
        //     }
        //     if let Ok(lib) = Library::new(lib_path) {
        //         library = Some(lib);
        //     } else {
        //         library = None;
        //     }
        // } else {
        //     library = None;
        // }
        // let library2: Option<Library>;
        // if let Some(lib_path2) = env::args().nth(2) {
        //     if cfg!(windows) && !lib_path2.ends_with(".dll") {
        //         panic!("Must use .dll if running an AI in windows!")
        //     } else if cfg!(unix) && !lib_path2.ends_with(".so") {
        //         panic!("Must use .so if running an AI in a 'nix-system!")
        //     }
        //     if let Ok(lib2) = Library::new(lib_path2) {
        //         library2 = Some(lib2);
        //     } else {
        //         library2 = None;
        //     }
        // } else {
        //     library2 = None;
        // }

        let mut players = [Player::new(init_level), Player::new(init_level)];
        // open channel for multi-threading
        let (sender, recieved_moves) = mpsc::channel();
        let (moves_to_send, reciever) = mpsc::channel();

        let mut stream: TcpStream;
        let connection_type = env::args().nth(1).unwrap();
        let ip = env::args().nth(2).unwrap();
        // establish connection
        if  connection_type == "host" {
            let listener = TcpListener::bind(ip).expect("invalid adress");
            stream = listener.accept().unwrap().0;
        } else if connection_type == "connect" {
            stream = TcpStream::connect(ip).unwrap();
        } else {
            panic!("Could not establish connection");
        }

        // connection established
        // send first package containing current piece and next piece
        stream
            .write(&[
                players[0].current_piece.color as u8,
                players[0].next_piece.color as u8,
            ])
            .unwrap();
        stream.flush().unwrap();

        // read first package
        let mut buffer = [0; 2];
        stream.read(&mut buffer);

        // set P2 pieces according to package
        let current_piece = Game::piece_from_u8(buffer[0]);
        let next_piece = Game::piece_from_u8(buffer[1]);
        players[1].current_piece = current_piece;
        players[1].next_piece = next_piece;

        // spawn thread
        thread::spawn(move || {
            Game::handle_connection(stream, sender, reciever);
        });

        Game {
            players: players,
            ai_lib: [None, None], //[library2, library],
            recieved_moves: recieved_moves,
            moves_to_send: moves_to_send,
        }
    }
    /// The game-tick update function
    pub fn update(&mut self) {
        // get next action from remote opponent
        if let Ok(package) = self.recieved_moves.try_recv() {
            if package != [0, 0] {
                //println!("Got a non [0,0] package");
                self.players[1].next_piece = Game::piece_from_u8(package[1]);
                self.parse_ai_output(1, package[0] as u32);
            }
        }
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
            let mut move_index = 0;
            match key {
                // P1 controlls
                KeyCode::A => {
                    self.players[0].move_current(-1, 0);
                    move_index = 1;
                }
                KeyCode::E => {
                    self.players[0].rotate_current(true);
                    move_index = 3;
                }
                KeyCode::D => {
                    self.players[0].move_current(1, 0);
                    move_index = 2;
                }
                KeyCode::Q => {
                    self.players[0].rotate_current(false);
                    move_index = 4;
                }
                KeyCode::S => {
                    self.players[0].move_current(0, -1);
                    move_index = 5;
                }
                KeyCode::W => {
                    self.players[0].drop_current();
                    move_index = 6;
                }
                KeyCode::Space => {
                    self.players[0].save_piece();
                    move_index = 7;
                }
                _ => (),
            }
            //println!("sent {:?} to the thread",[move_index, self.players[0].next_piece.color as u8] );
            // send move to opponent
            self.moves_to_send
                .send([move_index, self.players[0].next_piece.color as u8])
                .expect("move send error");
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
    /// handles recieving and sending moves to the online opponent
    fn handle_connection(
        mut stream: TcpStream,
        sender: mpsc::Sender<Packet>,
        reciever: mpsc::Receiver<Packet>,
    ) {
        println!("handeling connection");
        let mut buffer: Packet = [0; 2];
        let mut open = true;
        while open {
            if let Ok(response) = reciever.try_recv() {
                //println!("recieved move from main thread");
                stream.write(&response).unwrap();
                stream.flush().unwrap();
            //println!("wrote move from main thread");
            } else {
                stream.write(&[0, 0]).unwrap();
                stream.flush().unwrap();
            }

            match stream.read(&mut buffer) {
                Ok(0) => {
                    open = false;
                    println!("Connection closed!");
                }
                Ok(len) => {
                    if buffer != [0, 0] {
                        //println!("Recieved: {:?}", &buffer);
                    }
                }
                Err(_) => println!("Error reading stream"),
            }
            if buffer != [0, 0] {
                sender.send(buffer).expect("Send error");
            }
        }
    }
    /// creates a piece based on the color
    fn piece_from_u8(input: u8) -> Piece {
        let color = match input {
            1 => Color::Color1,
            2 => Color::Color2,
            3 => Color::Color3,
            4 => Color::Color4,
            5 => Color::Color5,
            6 => Color::Color6,
            _ => Color::Color7,
        };
        Piece::new(
            SHAPES[(input - 1) as usize],
            color,
            [COLS as i32 / 2, ROWS as i32 - 1],
        )
    }
}
