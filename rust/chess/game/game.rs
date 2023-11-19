
use utils::command_parser::CommandParser;

use super::error;
use super::error::ChessError;

use super::board::Location;
use super::board::Board;
use std::io;
use std::io::Write;

pub struct Game
{
    board: Board,
    exit_requested : bool,
    help_requested : bool
}

impl Game
{
    pub fn default() -> Game
    {
        Game
        {
            board: Board::default(),
            exit_requested: false,
            help_requested: false
        }
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

        self.board.try_move(from, to)?;

        //Draw board after a successfull move
        Game::draw_board(&self.board);
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