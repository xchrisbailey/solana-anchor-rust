use anchor_lang::prelude::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tic_tac_toe {
    use super::*;
    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> ProgramResult {
        let game = &mut ctx.accounts.game;
        game.players = [ctx.accounts.player_one.key(), player_two];
        game.turn = 1;
        Ok(())
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> ProgramResult {
        let game = &mut ctx.accounts.game;

        require!(
            game.current_player() == ctx.accounts.player.key(),
            TicTacToeError::NotPlayersTurn
        );

        game.play(&tile)
    }
}

#[account]
#[derive(Default)]
pub struct Game {
    players: [Pubkey; 2],
    turn: u8,
    board: [[Option<Sign>; 3]; 3],
    state: GameState,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Clone, Copy, PartialEq, Eq,
)]
pub enum Sign {
    X,
    O,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one, space = Game::MAXIMUM_SIZE + 8)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
}

#[error]
pub enum TicTacToeError {
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyOver,
    NotPlayersTurn,
}

impl Game {
    const MAXIMUM_SIZE: usize = 116;

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }

    fn current_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    pub fn current_player(&self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    pub fn play(&mut self, tile: &Tile) -> ProgramResult {
        if !self.is_active() {
            return Err(TicTacToeError::GameAlreadyOver.into());
        }
        match tile {
            tile @ Tile {
                row: 0..=2,
                column: 0..=2,
            } => match self.board[tile.row as usize][tile.column as usize] {
                Some(_) => return Err(TicTacToeError::TileAlreadySet.into()),
                None => {
                    self.board[tile.row as usize][tile.column as usize] =
                        Some(Sign::from_usize(self.current_player_index()).unwrap());
                }
            },
            _ => return Err(TicTacToeError::TileOutOfBounds.into()),
        }

        self.update_state();

        if let GameState::Active = self.state {
            self.turn += 1;
        }

        Ok(())
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        self.board[first.0][first.1].is_some()
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[second.0][second.1] == self.board[third.0][third.1]
    }

    fn update_state(&mut self) {
        for i in 0..=2 {
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }

            if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }

            if self.is_winning_trio([(0, 0), (1, 1), (2, 2)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }

            for row in 0..=2 {
                for column in 0..=2 {
                    if self.board[row][column].is_none() {
                        return;
                    }
                }
            }

            self.state = GameState::Tie;
        }
    }
}
