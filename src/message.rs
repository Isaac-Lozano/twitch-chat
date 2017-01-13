use std::collections::HashMap;
use std::str::FromStr;

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct Message
{
    pub raw: String,
    pub tags: HashMap<String, String>,
    pub from: Option<String>,
    pub cmd: String,
    pub args: Vec<String>,
}

impl FromStr for Message
{
    type Err = MessageError;
    fn from_str(s: &str) -> MessageResult<Self>
    {
        let mut line = String::from(s);
        let original_line = line.clone();
        let mut tags = HashMap::new();
        let mut from = None;
        let cmd;
        let mut args = Vec::new();

        let mut new_line;

        /* parse tags */
        if line.starts_with("@")
        {
            {
                let split: Vec<&str> = line[1..].splitn(2, ' ').collect();

                /* XXX */
                let tag_str = split.get(0).unwrap();
                new_line = split.get(1).unwrap().to_string();

                for tag in tag_str.split(';')
                {
                    let mut tag_split = tag.split('=');

                    let key = tag_split.next().unwrap();
                    let value = tag_split.next().unwrap();

                    tags.insert(key.to_string(), value.to_string());
                }
            }

            line = new_line;
        }

        /* parse from */
        if line.starts_with(":")
        {
            {
                let mut from_split = line[1..].splitn(2, ' ');

                /* XXX */
                from = Some(from_split.next().unwrap().to_string());
                new_line = from_split.next().unwrap().to_string();
            }

            line = new_line;
        }

        /* parse command */
        {
            let mut cmd_split = line.splitn(2, ' ');
            cmd = cmd_split.next().unwrap().to_string();

            new_line = cmd_split.next().unwrap_or("").to_string();
        }
        line = new_line;

        /* parse args */
        {
            let mut arg_split = line.split(' ');
            loop
            {
                match arg_split.next()
                {
                    Some(val) =>
                    {
                        if val.starts_with(':')
                        {
                            let rest = arg_split.fold(String::from(""), |acc, s| acc + " " + s);
                            args.push(val[1..].to_string() + &rest);
                            break;
                        }
                        else
                        {
                            args.push(val.to_string());
                        }
                    }
                    None =>
                        break,
                }
            }
        }

        Ok(
            Message
            {
                raw: original_line,
                tags: tags,
                from: from,
                cmd: cmd,
                args: args,
            }
        )
    }
}

#[derive(Debug)]
pub struct MessageError(&'static str);

impl fmt::Display for MessageError
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
    {
        write!(fmt, "MessageError: {}", self.description())
    }
}

impl Error for MessageError
{
    fn description(&self) -> &str
    {
        self.0
    }
}

pub type MessageResult<T> = Result<T, MessageError>;
