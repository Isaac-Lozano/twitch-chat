use auth::Auth;

use message::{Message, MessageError};

use websocket::{Client, Sender, Receiver};
use websocket::Message as WsMessage;
use websocket::sender::Sender as WsSender;
use websocket::receiver::Receiver as WsReceiver;
use websocket::stream::WebSocketStream;
use websocket::client::request::Url;
use websocket::message::Type;
use websocket::result::WebSocketError;

use std::str;
use std::str::FromStr;

use std::fmt;
use std::error::Error;

pub struct ChatClient
{
    sender: WsSender<WebSocketStream>,
    receiver: WsReceiver<WebSocketStream>,
}

impl ChatClient
{
    pub fn connect() -> ClientResult<ChatClient>
    {
        let chat_url = Url::parse("wss://irc-ws.chat.twitch.tv/").unwrap();
        let request = try!(Client::connect(chat_url));
        let response = try!(request.send());
        try!(response.validate());

        let client = response.begin();
        let (sender, receiver) = client.split();

        Ok(ChatClient
        {
            sender: sender,
            receiver: receiver,
        })
    }

    pub fn reconnect(&mut self) -> ClientResult<()>
    {
        let chat_url = Url::parse("wss://irc-ws.chat.twitch.tv/").unwrap();
        let request = try!(Client::connect(chat_url));
        let response = try!(request.send());
        try!(response.validate());

        let client = response.begin();
        let (sender, receiver) = client.split();

        self.sender = sender;
        self.receiver = receiver;
        Ok(())
    }

    pub fn split(self) -> (ChatSender, ChatReceiver)
    {
        (
            ChatSender
            {
                sender: self.sender,
            },
            ChatReceiver
            {
                receiver: self.receiver,
            }
        )
    }
}

impl TwitchSender for ChatClient
{
    fn send_raw(&mut self, message: &str) -> ClientResult<()>
    {
        try!(self.sender.send_message(&WsMessage::text(message)));
        Ok(())
    }
}

impl TwitchReceiver for ChatClient
{
    fn get_message(&mut self) -> ClientResult<Message>
    {
        let msg: WsMessage = try!(self.receiver.recv_message());

        loop
        {
            match msg.opcode
            {
                Type::Text =>
                {
                    return Ok(Message::from_str(str::from_utf8(&*msg.payload).unwrap()).unwrap());
                }
                /* TODO: Error? */
                _ => {},
            }
        }
    }
}

pub struct ChatSender
{
    sender: WsSender<WebSocketStream>,
}

impl TwitchSender for ChatSender
{
    fn send_raw(&mut self, message: &str) -> ClientResult<()>
    {
        try!(self.sender.send_message(&WsMessage::text(message)));
        Ok(())
    }
}

pub struct ChatReceiver
{
    receiver: WsReceiver<WebSocketStream>,
}

impl TwitchReceiver for ChatReceiver
{
    fn get_message(&mut self) -> ClientResult<Message>
    {
        let msg: WsMessage = try!(self.receiver.recv_message());

        loop
        {
            match msg.opcode
            {
                Type::Text =>
                {
                    return Ok(Message::from_str(str::from_utf8(&*msg.payload).unwrap()).unwrap());
                }
                /* TODO: Error? */
                _ => {},
            }
        }
    }
}

pub trait TwitchSender
{
    fn send_raw(&mut self, message: &str) -> ClientResult<()>;

    fn send_authenticate(&mut self, auth_opt: Option<Auth>) -> ClientResult<()>
    {
        try!(self.send_raw("CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership"));
        if let Some(auth) = auth_opt
        {
            try!(self.send_raw(&format!("PASS {}", auth.oauth)));
            try!(self.send_raw(&format!("NICK {}", auth.username)));
        }
        else
        {
            try!(self.send_raw("NICK justinfan1"));
        }
        Ok(())
    }

    fn send_join(&mut self, channel: &str) -> ClientResult<()>
    {
        /* TODO: lowercase and strip '#'*/
        self.send_raw(&format!("JOIN {}", channel))
    }

    fn send_message(&mut self, channel: &str, message: &str) -> ClientResult<()>
    {
        self.send_raw(&format!("PRIVMSG {} :{}", channel, message))
    }
}

pub trait TwitchReceiver
{
    fn get_message(&mut self) -> ClientResult<Message>;
}

#[derive(Debug)]
pub enum ClientError
{
    MessageError(MessageError),
    Utf8Error(str::Utf8Error),
    WebSocketError(WebSocketError),
}

impl fmt::Display for ClientError
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result
    {
        write!(fmt, "ClientError: {}", self.description())
    }
}

impl Error for ClientError
{
    fn description(&self) -> &str
    {
        match *self
        {
            ClientError::MessageError(_) => "Error parsing incoming message",
            ClientError::Utf8Error(_) => "UTF-8 error while parsing message",
            ClientError::WebSocketError(_) => "Websocket error",
        }
    }

    fn cause(&self) -> Option<&Error>
    {
        match *self
        {
            ClientError::MessageError(ref e) => Some(e),
            ClientError::Utf8Error(ref e) => Some(e),
            ClientError::WebSocketError(ref e) => Some(e),
        }
    }
}

impl From<MessageError> for ClientError
{
    fn from(e: MessageError) -> ClientError
    {
        ClientError::MessageError(e)
    }
}

impl From<str::Utf8Error> for ClientError
{
    fn from(e: str::Utf8Error) -> ClientError
    {
        ClientError::Utf8Error(e)
    }
}

impl From<WebSocketError> for ClientError
{
    fn from(e: WebSocketError) -> ClientError
    {
        ClientError::WebSocketError(e)
    }
}

type ClientResult<T> = Result<T, ClientError>;
