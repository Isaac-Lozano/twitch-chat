use auth::Auth;

use message::Message;

use websocket::{Client, Sender, Receiver};
use websocket::Message as WsMessage;
use websocket::sender::Sender as WsSender;
use websocket::receiver::Receiver as WsReceiver;
use websocket::stream::WebSocketStream;
use websocket::client::request::Url;
use websocket::message::Type;

use std::str;
use std::str::FromStr;

pub struct ChatClient
{
    sender: WsSender<WebSocketStream>,
    receiver: WsReceiver<WebSocketStream>,
}

impl ChatClient
{
    pub fn new() -> ChatClient
    {
        let chat_url = Url::parse("wss://irc-ws.chat.twitch.tv/").unwrap();
        let request = Client::connect(chat_url).unwrap();
        let response = request.send().unwrap();
        response.validate().unwrap();
        let client = response.begin();
        let (sender, receiver) = client.split();

        ChatClient
        {
            sender: sender,
            receiver: receiver,
        }
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
    fn send_raw(&mut self, message: &str)
    {
        self.sender.send_message(&WsMessage::text(message)).unwrap();
    }
}

impl TwitchReceiver for ChatClient
{
    fn get_message(&mut self) -> Message
    {
        let msg: WsMessage = match self.receiver.recv_message()
        {
            Ok(m) => m,
            /* TODO: Error */
            Err(e) => return Message::from_str("").unwrap(),
        };

        match msg.opcode
        {
            Type::Text =>
            {
                Message::from_str(str::from_utf8(&*msg.payload).unwrap()).unwrap()
            }
            /* TODO: ERROR */
            _ => return Message::from_str("").unwrap(),
        }
    }
}

pub struct ChatSender
{
    sender: WsSender<WebSocketStream>,
}

impl TwitchSender for ChatSender
{
    fn send_raw(&mut self, message: &str)
    {
        self.sender.send_message(&WsMessage::text(message)).unwrap();
    }
}

pub struct ChatReceiver
{
    receiver: WsReceiver<WebSocketStream>,
}

impl TwitchReceiver for ChatReceiver
{
    fn get_message(&mut self) -> Message
    {
        let msg: WsMessage = match self.receiver.recv_message()
        {
            Ok(m) => m,
            /* TODO: Error */
            Err(e) => return Message::from_str("").unwrap(),
        };

        match msg.opcode
        {
            Type::Text =>
            {
                Message::from_str(str::from_utf8(&*msg.payload).unwrap()).unwrap()
            }
            /* TODO: ERROR */
            _ => return Message::from_str("").unwrap(),
        }
    }
}

pub trait TwitchSender
{
    fn send_raw(&mut self, message: &str);

    fn send_authenticate(&mut self, auth_opt: Option<Auth>)
    {
        let auth = auth_opt.unwrap_or(Auth::new("jutinfan1", "blah"));
        self.send_raw("CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership");
        self.send_raw(&format!("PASS {}", auth.oauth));
        self.send_raw(&format!("NICK {}", auth.username));
    }

    fn send_join(&mut self, channel: &str)
    {
        /* TODO: lowercase and strip '#'*/
        self.send_raw(&format!("JOIN {}", channel));
    }

    fn send_message(&mut self, channel: &str, message: &str)
    {
        self.send_raw(&format!("PRIVMSG {} :{}", channel, message));
    }
}

pub trait TwitchReceiver
{
    fn get_message(&mut self) -> Message;
}
