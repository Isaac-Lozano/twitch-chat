extern crate twitch_websocket;

use twitch_websocket::client::{ChatClient, TwitchSender, TwitchReceiver};
use twitch_websocket::auth::Auth;

fn main() {
    println!("Making client.");
    let mut client = ChatClient::new();
    println!("Authenticating.");
    client.send_authenticate(None));
    println!("Joining.");
    client.send_join("#onvar");

    let (mut sender, mut receiver) = client.split();

    println!("Getting messages.");
    loop
    {
        let message = receiver.get_message();
        println!("<< {}", message.raw);
        match message.cmd.as_str()
        {
            "PRIVMSG" =>
            {
                if let Some(channel) = message.args.get(0)
                {
                    if let Some(message) = message.args.get(1)
                    {
                        let out_str = format!("You said: {}", message);
                        println!(">> {}", out_str);
                        sender.send_message(channel, &out_str);
                    }
                }
            },
            _ => {}
        }
    }
}
