use crate::game_state::Game;

use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Font, Mesh, MeshBuilder, Rect, Scale, Text};

use ggez::{Context, GameResult};
use graphics::TextFragment;

/// size of the window
pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

/// Visible size of the tetris board
pub const GRID_SIZE: (i32, i32) = (10, 20);
const GRID_LINE_WIDTH: f32 = 1.0;

/// Size of each block
pub const BLOCK_SIZE: (f32, f32) = (20.0, 20.0);

/// Size of the scaled-down blocks
const SMALL_BLOCK_SIZE: (f32, f32) = (BLOCK_SIZE.0 * 0.5, BLOCK_SIZE.1 * 0.5);

/// The top-left corner of the boards
pub const P1_BOARD_PLACEMENT: (f32, f32) = (100.0, 100.0);
pub const P2_BOARD_PLACEMENT: (f32, f32) = (SCREEN_SIZE.0 / 2.0 + 100.0, 100.0);

/// The x y w h of the boards
pub const P1_BOARD: (f32, f32, f32, f32) = (
    P1_BOARD_PLACEMENT.0,                // x
    P1_BOARD_PLACEMENT.1,                // y
    (GRID_SIZE.0 as f32) * BLOCK_SIZE.0, // width
    (GRID_SIZE.1 as f32) * BLOCK_SIZE.0, // height
);
/// The x y w h of the boards
pub const P2_BOARD: (f32, f32, f32, f32) = (
    P2_BOARD_PLACEMENT.0,                // x
    P2_BOARD_PLACEMENT.1,                // y
    (GRID_SIZE.0 as f32) * BLOCK_SIZE.0, // width
    (GRID_SIZE.1 as f32) * BLOCK_SIZE.0, // height
);

// for the next piece and saved piece boxes
const INFO_BOX: (f32, f32) = (SMALL_BLOCK_SIZE.0 * 6.0, SMALL_BLOCK_SIZE.1 * 6.0);
const INFO_BOX_MARGIN: (f32, f32) = (SMALL_BLOCK_SIZE.0, SMALL_BLOCK_SIZE.1);

// size of the attack meter increments
const ATTACK_METER: (f32, f32) = (BLOCK_SIZE.0 / 2.0, BLOCK_SIZE.1);

// the center of the score text
const P1_SCORE_PLACEMENT: (f32, f32) = (
    P1_BOARD.0 + P1_BOARD.2 / 2.0,
    P1_BOARD.1 + P1_BOARD.3 + 30.0,
);
const P2_SCORE_PLACEMENT: (f32, f32) = (
    P2_BOARD.0 + P2_BOARD.2 / 2.0,
    P2_BOARD.1 + P2_BOARD.3 + 30.0,
);

const BACKGROUND_COLOR: Color = Color::new(25.0 / 255.0, 172.0 / 255.0, 244.0 / 255.0, 1.0);
const BOARD_BACKGROUND: Color = Color::new(0.0, 0.0, 0.0, 0.8);
const GRID_COLOR: Color = Color::new(100.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 1.0);

pub const PALETTE: [Color; 8] = [
    Color::new(0.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 1.0), // Cyan
    Color::new(255.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 1.0), // Yellow
    Color::new(128.0 / 255.0, 0.0 / 255.0, 128.0 / 255.0, 1.0), // Purple
    Color::new(0.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 1.0),   // Green
    Color::new(255.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0, 1.0),   // Red
    Color::new(0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0, 1.0),   // Blue
    Color::new(255.0 / 255.0, 127.0 / 255.0, 0.0 / 255.0, 1.0), // Orange
    Color::new(127.0 / 255.0, 127.0 / 255.0, 127.0 / 255.0, 1.0), // Grey
];

pub const GHOST_PALETTE: [Color; 7] = [
    Color::new(0.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.3), // Cyan
    Color::new(255.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 0.3), // Yellow
    Color::new(128.0 / 255.0, 0.0 / 255.0, 128.0 / 255.0, 0.5), // Purple
    Color::new(0.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 0.3),   // Green
    Color::new(255.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0, 0.3),   // Red
    Color::new(0.0 / 255.0, 0.0 / 255.0, 175.0 / 255.0, 0.5),   // Blue
    Color::new(255.0 / 255.0, 127.0 / 255.0, 0.0 / 255.0, 0.3), // Orange
];

const INIT_LEVEL: usize = 5;

// contains fields like the game struct, ai-script, etc. Basically stores the game-state + resources
pub struct AppState {
    game_state: Game,
    block_palatte: [Mesh; 15],
    grid_mesh: Mesh,
    small_block_palatte: [Mesh; 8],
    font: Font,
}

impl AppState {
    pub fn new(ctx: &mut Context) -> AppState {
        let state = AppState {
            // Load/create resources here: images, fonts, sounds, etc.
            game_state: Game::new(5),
            block_palatte: generate_blocks(ctx),
            grid_mesh: generate_grid_mesh(ctx).expect("grid mesh err"),
            small_block_palatte: generate_small_blocks(ctx),
            font: Font::new(ctx, "/Roboto-Regular.ttf").expect("font loading error"),
        };
        state
    }
}

impl event::EventHandler for AppState {
    // update the game logic
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let [p1_lost, p2_lost] = self.game_state.get_losts();

        if p1_lost || p2_lost {
            // if either player has lost
        } else {
            self.game_state.update();
        }
        Ok(())
    }

    // update the graphics
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear screen with the background color
        graphics::clear(ctx, BACKGROUND_COLOR);

        // draw boards
        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, P1_BOARD.2 as i32, P1_BOARD.3 as i32),
            BOARD_BACKGROUND,
        )?;

        graphics::draw(
            ctx,
            &rectangle,
            (ggez::mint::Point2 {
                x: P1_BOARD.0,
                y: P1_BOARD.1,
            },),
        )?;

        graphics::draw(
            ctx,
            &rectangle,
            (ggez::mint::Point2 {
                x: P2_BOARD.0,
                y: P2_BOARD.1,
            },),
        )?;

        // draw next piece boxes
        let info_box = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, INFO_BOX.0 as i32, INFO_BOX.1 as i32),
            BOARD_BACKGROUND,
        )?;

        graphics::draw(
            ctx,
            &info_box,
            (ggez::mint::Point2 {
                x: P1_BOARD.0 + P1_BOARD.2,
                y: P1_BOARD.1,
            },),
        )?;

        graphics::draw(
            ctx,
            &info_box,
            (ggez::mint::Point2 {
                x: P2_BOARD.0 + P2_BOARD.2,
                y: P2_BOARD.1,
            },),
        )?;

        // draw saved piece boxes
        graphics::draw(
            ctx,
            &info_box,
            (ggez::mint::Point2 {
                x: P1_BOARD.0 - INFO_BOX.0,
                y: P1_BOARD.1,
            },),
        )?;

        graphics::draw(
            ctx,
            &info_box,
            (ggez::mint::Point2 {
                x: P2_BOARD.0 - INFO_BOX.0,
                y: P2_BOARD.1,
            },),
        )?;

        // draw next pieces
        let [p1_next_piece, p2_next_piece] = self.game_state.get_next_pieces();

        for y in 0..p1_next_piece.len() {
            for x in 0..p1_next_piece[y].len() {
                if p1_next_piece[y][x] > 0 {
                    graphics::draw(
                        ctx,
                        &self.small_block_palatte[p1_next_piece[y][x] as usize - 1],
                        (ggez::mint::Point2 {
                            x: x as f32 * SMALL_BLOCK_SIZE.0
                                + P1_BOARD.0
                                + P1_BOARD.2
                                + INFO_BOX_MARGIN.0,
                            y: y as f32 * SMALL_BLOCK_SIZE.1 + P1_BOARD.1 + INFO_BOX_MARGIN.1,
                        },),
                    )?
                }
            }
        }

        for y in 0..p2_next_piece.len() {
            for x in 0..p2_next_piece[y].len() {
                if p2_next_piece[y][x] > 0 {
                    graphics::draw(
                        ctx,
                        &self.small_block_palatte[p2_next_piece[y][x] as usize - 1],
                        (ggez::mint::Point2 {
                            x: x as f32 * SMALL_BLOCK_SIZE.0
                                + P2_BOARD.0
                                + P2_BOARD.2
                                + INFO_BOX_MARGIN.0,
                            y: y as f32 * SMALL_BLOCK_SIZE.1 + P2_BOARD.1 + INFO_BOX_MARGIN.1,
                        },),
                    )?
                }
            }
        }

        // draw saved pieces
        let [p1_saved_piece, p2_saved_piece] = self.game_state.get_saved_pieces();

        for y in 0..p1_saved_piece.len() {
            for x in 0..p1_saved_piece[y].len() {
                if p1_saved_piece[y][x] > 0 {
                    graphics::draw(
                        ctx,
                        &self.small_block_palatte[p1_saved_piece[y][x] as usize - 1],
                        (ggez::mint::Point2 {
                            x: x as f32 * SMALL_BLOCK_SIZE.0 + P1_BOARD.0 - INFO_BOX.0
                                + INFO_BOX_MARGIN.0,
                            y: y as f32 * SMALL_BLOCK_SIZE.1 + P1_BOARD.1 + INFO_BOX_MARGIN.1,
                        },),
                    )?
                }
            }
        }

        for y in 0..p2_saved_piece.len() {
            for x in 0..p2_saved_piece[y].len() {
                if p2_saved_piece[y][x] > 0 {
                    graphics::draw(
                        ctx,
                        &self.small_block_palatte[p2_saved_piece[y][x] as usize - 1],
                        (ggez::mint::Point2 {
                            x: x as f32 * SMALL_BLOCK_SIZE.0 + P2_BOARD.0 - INFO_BOX.0
                                + INFO_BOX_MARGIN.0,
                            y: y as f32 * SMALL_BLOCK_SIZE.1 + P2_BOARD.1 + INFO_BOX_MARGIN.1,
                        },),
                    )?
                }
            }
        }
        let boards = self.game_state.get_boards();
        let p1_board = boards[0];
        let p2_board = boards[1];

        // draw blocks
        for y in 0..(p1_board.len() - 4) {
            for x in 0..p1_board[y].len() {
                if p1_board[y][x] > 0 {
                    graphics::draw(
                        ctx,
                        &self.block_palatte[p1_board[y][x] as usize - 1],
                        (ggez::mint::Point2 {
                            x: P1_BOARD.0 + (x as f32) * BLOCK_SIZE.0,
                            y: P1_BOARD.1 + P1_BOARD.3 - ((y as f32) + 1.0) * BLOCK_SIZE.1,
                        },),
                    )
                    .expect("msg");
                }
            }
        }

        for y in 0..(p2_board.len() - 4) {
            for x in 0..p2_board[y].len() {
                if p2_board[y][x] > 0 {
                    graphics::draw(
                        ctx,
                        &self.block_palatte[p2_board[y][x] as usize - 1],
                        (ggez::mint::Point2 {
                            x: P2_BOARD.0 + (x as f32) * BLOCK_SIZE.0,
                            y: P2_BOARD.1 + P2_BOARD.3 - ((y as f32) + 1.0) * BLOCK_SIZE.1,
                        },),
                    )
                    .expect("msg");
                }
            }
        }

        // draw attack meters
        let [p1_meter, p2_meter] = self.game_state.get_attackbars();

        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, ATTACK_METER.0 as i32, ATTACK_METER.1 as i32),
            PALETTE[7],
        )?;

        for i in 1..(p1_meter + 1) {
            graphics::draw(
                ctx,
                &rectangle,
                (ggez::mint::Point2 {
                    x: P1_BOARD.0 - ATTACK_METER.0,
                    y: P1_BOARD.1 + P1_BOARD.3 - i as f32 * ATTACK_METER.1,
                },),
            )?;
        }

        for i in 1..(p2_meter + 1) {
            graphics::draw(
                ctx,
                &rectangle,
                (ggez::mint::Point2 {
                    x: P2_BOARD.0 - ATTACK_METER.0,
                    y: P2_BOARD.1 + P2_BOARD.3 - i as f32 * ATTACK_METER.1,
                },),
            )?;
        }

        // draw grids
        graphics::draw(
            ctx,
            &self.grid_mesh,
            (ggez::mint::Point2 {
                x: P1_BOARD.0,
                y: P1_BOARD.1,
            },),
        )?;
        graphics::draw(
            ctx,
            &self.grid_mesh,
            (ggez::mint::Point2 {
                x: P2_BOARD.0,
                y: P2_BOARD.1,
            },),
        )?;

        // draw text
        let scores = self.game_state.get_scores();
        let p1_score = TextFragment::new(scores[0].to_string())
            .font(self.font)
            .scale(Scale { x: 25.0, y: 25.0 });
        let p1_score_text = Text::new(p1_score);
        let p1_dimensions = p1_score_text.dimensions(ctx);
        let p2_score = TextFragment::new(scores[1].to_string())
            .font(self.font)
            .scale(Scale { x: 25.0, y: 25.0 });
        let p2_score_text = Text::new(p2_score);
        let p2_dimensions = p1_score_text.dimensions(ctx);

        graphics::draw(
            ctx,
            &p1_score_text,
            (ggez::mint::Point2 {
                x: P1_SCORE_PLACEMENT.0 - (p1_dimensions.0 as f32) / 2.0,
                y: P1_SCORE_PLACEMENT.1 - (p1_dimensions.1 as f32) / 2.0,
            },),
        )?;

        graphics::draw(
            ctx,
            &p2_score_text,
            (ggez::mint::Point2 {
                x: P2_SCORE_PLACEMENT.0 - (p2_dimensions.0 as f32) / 2.0,
                y: P2_SCORE_PLACEMENT.1 - (p2_dimensions.1 as f32) / 2.0,
            },),
        )?;

        // if anyone lost draw
        let [p1_lost, p2_lost] = self.game_state.get_losts();

        if p1_lost {
            let p2_win = TextFragment::new("P2 wins!")
                .font(self.font)
                .scale(Scale { x: 100.0, y: 100.0 });
            let p2_win_text = Text::new(p2_win);
            let dimensions = p2_win_text.dimensions(ctx);

            graphics::draw(
                ctx,
                &p2_win_text,
                (ggez::mint::Point2 {
                    x: SCREEN_SIZE.0 / 2.0 - (dimensions.0 as f32) / 2.0,
                    y: SCREEN_SIZE.1 / 2.0 - (dimensions.1 as f32) / 2.0,
                },),
            )?;
        } else if p2_lost {
            let p1_win = TextFragment::new("P1 wins!")
                .font(self.font)
                .scale(Scale { x: 100.0, y: 100.0 });
            let p1_win_text = Text::new(p1_win);
            let dimensions = p1_win_text.dimensions(ctx);

            graphics::draw(
                ctx,
                &p1_win_text,
                (ggez::mint::Point2 {
                    x: SCREEN_SIZE.0 / 2.0 - (dimensions.0 as f32) / 2.0,
                    y: SCREEN_SIZE.1 / 2.0 - (dimensions.1 as f32) / 2.0,
                },),
            )?;
        }

        // present the graphics to the graphics engine
        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::R {
            self.game_state.restart(INIT_LEVEL);
        } else {
            self.game_state.key_down(keycode);
        }
    }
}
/// Generates the meshes for the tetromino block
fn generate_blocks(ctx: &mut Context) -> [Mesh; 15] {
    [
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[0],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[1],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[2],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[3],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[4],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[5],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[6],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[7],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            GHOST_PALETTE[0],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            GHOST_PALETTE[1],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            GHOST_PALETTE[2],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            GHOST_PALETTE[3],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            GHOST_PALETTE[4],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            GHOST_PALETTE[5],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            GHOST_PALETTE[6],
        )
        .expect("Failed creating blocks"),
    ]
}
/// generates the mesh for the grid-lines
fn generate_grid_mesh(ctx: &mut Context) -> GameResult<Mesh> {
    let mut mesh = MeshBuilder::new();
    for x in 0..(GRID_SIZE.0 + 1) {
        mesh.line(
            &[
                ggez::mint::Point2 {
                    x: (x as f32) * BLOCK_SIZE.0,
                    y: 0.0,
                },
                ggez::mint::Point2 {
                    x: (x as f32) * BLOCK_SIZE.0,
                    y: P1_BOARD.3,
                },
            ],
            GRID_LINE_WIDTH,
            GRID_COLOR,
        )
        .expect("msg");
    }
    for y in 0..(GRID_SIZE.1 + 1) {
        mesh.line(
            &[
                ggez::mint::Point2 {
                    x: 0.0,
                    y: (y as f32) * BLOCK_SIZE.1,
                },
                ggez::mint::Point2 {
                    x: P1_BOARD.2,
                    y: (y as f32) * BLOCK_SIZE.1,
                },
            ],
            GRID_LINE_WIDTH,
            GRID_COLOR,
        )
        .expect("msg");
    }

    mesh.build(ctx)
}

/// generates the meshes for the scaled-down tetromino for next_piece and saved_piece
fn generate_small_blocks(ctx: &mut Context) -> [Mesh; 8] {
    [
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, SMALL_BLOCK_SIZE.0 as i32, SMALL_BLOCK_SIZE.1 as i32),
            PALETTE[0],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, SMALL_BLOCK_SIZE.0 as i32, SMALL_BLOCK_SIZE.1 as i32),
            PALETTE[1],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, SMALL_BLOCK_SIZE.0 as i32, SMALL_BLOCK_SIZE.1 as i32),
            PALETTE[2],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, SMALL_BLOCK_SIZE.0 as i32, SMALL_BLOCK_SIZE.1 as i32),
            PALETTE[3],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, SMALL_BLOCK_SIZE.0 as i32, SMALL_BLOCK_SIZE.1 as i32),
            PALETTE[4],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, SMALL_BLOCK_SIZE.0 as i32, SMALL_BLOCK_SIZE.1 as i32),
            PALETTE[5],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, SMALL_BLOCK_SIZE.0 as i32, SMALL_BLOCK_SIZE.1 as i32),
            PALETTE[6],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, SMALL_BLOCK_SIZE.0 as i32, SMALL_BLOCK_SIZE.1 as i32),
            PALETTE[7],
        )
        .expect("Failed creating blocks"),
    ]
}
mod tests {
    use super::{AppState, SCREEN_SIZE};
    use ggez::event::{self, EventHandler};
    use ggez::graphics;
    use ggez::{Context, ContextBuilder, GameResult};
    use std::path;

    #[test]
    fn window_test() {
        let resource_dir = path::PathBuf::from("./resources");
        let context_builder = ggez::ContextBuilder::new("tetris", "malte och isak")
            .add_resource_path(resource_dir)
            .window_setup(ggez::conf::WindowSetup::default().title("Test goes brrr"))
            .window_mode(
                ggez::conf::WindowMode::default()
                    .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimenstions
                    .resizable(true), // Fixate window size
            );

        let (contex, event_loop) = &mut context_builder.build().expect("context builder error");

        let state = &mut AppState::new(contex);

        event::run(contex, event_loop, state);
    }
}
