

pub struct CommandParser<'a>
{
    data: Vec<&'a str>,
    cur_idx: usize
}

impl CommandParser<'_>
{
    pub fn create<'a>(input: &'a str, delim: &'a str) -> CommandParser<'a>
    {
        CommandParser{
            data: input.split(delim).collect(),
            cur_idx: 0
        }
    }

    pub fn cur(&self) -> &str
    {
        if self.cur_idx < self.data.len()
        {
            self.data[self.cur_idx]
        }
        else 
        {
            ""
        }
    }

    pub fn next(& mut self) -> &str
    {
        self.cur_idx += 1;
        self.cur()
    }

    pub fn to_string(&self) -> String
    {
        let mut out_str = String::new();
        for str in &self.data
        {
            out_str += str;
            out_str += " ";
        }
        out_str
    }

}