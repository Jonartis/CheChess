
use utils::command_parser::CommandParser;

use super::error;
use super::error::ChessError;

use super::board::Location;
use super::board::Board;
use super::board::MovementResult;
use super::board::PieceType;

use std::io;
use std::io::Write;
use std::collections::HashMap;

pub struct Game
{
    board: Board,
    exit_requested : bool,
    help_requested : bool,
}

impl Game
{
    pub fn default() -> Game
    {
        Game
        {
            board: Board::create(),
            exit_requested: false,
            help_requested: false,
        }
    }
    
    pub fn init(&mut self)
    {
        Game::draw_board(&self.board);
    }

    pub fn update(&mut self)
    {
        let cmd = Game::request_input();

        match self.process_command(&cmd)
        {
            Err(error) => self.on_error(error),
            _ => ()
        }

        if self.help_requested
        {
            self.show_help();
        }
    }

    pub fn should_quit(&self) -> bool
    {
        self.exit_requested
    }
}

impl Game //Events
{
    fn on_print_board(&self)
    {
        Game::draw_board(&self.board);
    }

    fn on_quit(&mut self)
    {
        self.exit_requested = true;
    }
    
    fn on_invalid_command(&mut self, cmd : &str)
    {
        println!("Invalid command '{cmd}'");
        self.request_help();
    }

    fn on_move(&mut self, cmd_parse : &mut CommandParser) -> Result<(), ChessError>
    {
        let from = Location::try_from(cmd_parse.next())?;
        let to = Location::try_from(cmd_parse.next())?;

        let result = self.board.try_move(from, to)?;
        Game::draw_board(&self.board);
        match result {
            MovementResult::Moved => {
            }
            MovementResult::GameFinished => {
                println!("Game finished!");
                let winner_type = self.board.get_playing_player();
                println!("'{winner_type}' won!");
                self.exit_requested = true;
            }
            MovementResult::PawnUpgrade=> {
                let consumed_pieces = self.board.get_consumed_piece_types(self.board.get_playing_player());
                
                //Filter the available options and store them in a set with their display
                let upgrade_options: HashMap<&'static str, PieceType> = consumed_pieces.iter()
                    .filter(|(_, &available_count)| available_count > 0)
                    .map(|(piece_type, _)| (piece_type.create_behaviour().to_board_string(self.board.get_playing_player()), piece_type.clone()))
                    .collect();
                
                //Convert the options into a list separated by commas
                let options_as_str = upgrade_options.keys().copied().collect::<Vec<_>>().join(", ");
                
                let mut chosen_piece: Option<PieceType> = None;
                while chosen_piece == None && !upgrade_options.is_empty()
                {
                    clearscreen::clear().expect("failed to clear screen");
                    println!("Upgrade your pawn to one of these: {options_as_str}");

                    let cmd = Game::request_input();

                    let cmd_parse = CommandParser::create(&cmd, " ");
                    println!("Input: {}", cmd_parse.to_string());
                    //Check if the given piece is valid. We need to convert the enum to its string representation first
                    if let Some(entry) = upgrade_options.get(cmd_parse.cur()) {
                        chosen_piece = Some(entry.clone());
                    }
                }
                self.board.upgrade_pawn(to, chosen_piece.unwrap_or(PieceType::Pawn))?;
            }
            MovementResult::FailedToMove => {
                println!("Move failed!");
            }
        }

        Ok(())
    }

    fn on_error(&mut self, error: ChessError)
    {
        error::print_error(error);
        self.request_help();
    }

}

impl Game //Helpers
{
    fn process_command(&mut self, cmd : &str) -> Result<(), ChessError>
    {
        clearscreen::clear().expect("failed to clear screen");

        let mut cmd_parse = CommandParser::create(cmd, " ");
        println!("Input: {}", cmd_parse.to_string());

        match cmd_parse.cur()
        {
            "db" => self.on_print_board(),
            "q" => self.on_quit(),
            "mv" => self.on_move(&mut cmd_parse)?,
            "h" => self.request_help(),
            other => {self.on_invalid_command(other)}
        }
        Ok(())
    }

    fn request_help(&mut self)
    {
        self.help_requested = true;
    }

    fn show_help(&mut self)
    {
        self.help_requested = false;
        println!();
        println!("INSTRUCTIONS");
        println!("db: Draw the board");
        println!("mv: Move a piece from row,column to row,colum. Ex: mv 1a 3c");
        println!("h: Show this help");
        println!("q: Quit game");
    }

    fn request_input() -> String
    {
        println!();
        print!("Input Command: "); 
        let _ = io::stdout().flush();
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read command");

        user_input.trim_end().to_string()
    }

    fn draw_board(board : &Board)
    {
        println!();
        println!("{}", board);
    }

}