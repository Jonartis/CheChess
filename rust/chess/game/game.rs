
use utils::command_parser::CommandParser;
use crate::error::InputError;

use super::board::Location;
use super::board::Board;

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

    pub fn update(&mut self, cmd : &str)
    {
        self.process_command(cmd);

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

impl Game
{
    fn process_command(&mut self, cmd : &str)
    {
        clearscreen::clear().expect("failed to clear screen");

        let mut cmd_parse = CommandParser::create(cmd, " ");
        println!("Input: {}", cmd_parse.to_string());

        match cmd_parse.cur()
        {
            "db" => {self.on_print_board();},
            "q" => {self.on_quit();},
            "mv" => {self.on_move(&mut cmd_parse)},
            other => {self.on_invalid_command(other)}
        }
    }

    fn on_print_board(&self)
    {
        println!("Board \n{}", self.board);
    }

    fn on_quit(&mut self)
    {
        self.exit_requested = true;
    }
    
    fn on_invalid_command(&mut self, cmd : &str)
    {
        self.request_help(std::format!("Invalid command '{cmd}'"));
    }

    fn on_move(&mut self, cmd_parse : &mut CommandParser)
    {
        //We need to make a copy here because otherwise Rust can't guarantee we won't modify the reference
        //in the operation below that takes a mutable reference
        let from = match Location::try_from(cmd_parse.next())
        {
            Ok(loc) => loc,
            Err(err) => {self.on_error(err); return;}
        };
        let to = match Location::try_from(cmd_parse.next())
        {
            Ok(loc) => loc,
            Err(err) => {self.on_error(err); return;}
        };

        match self.board.make_move(from, to)
        {
            Err(error) => println!("Error making move {:?}", error),
            Ok(_) => self.on_print_board()
        }
    }

    fn request_help(&mut self, reason: String)
    {
        println!("Requesting help: {reason}");
        self.help_requested = true;
    }

    fn show_help(&mut self)
    {
        self.help_requested = false;
        println!("TODO: Write instructions");
    }

    fn on_error(&self, error: InputError)
    {
        println!("Error! {:?}", error);
    }

}