extern crate twitch_chat;

use twitch_chat::client::{ChatClient, TwitchSender, TwitchReceiver};

fn main() {
    println!("Making client.");
    let mut client = ChatClient::connect().unwrap();
    println!("Authenticating.");
    client.send_authenticate(None).unwrap();
    println!("Joining.");
    client.send_join("#onvar").unwrap();

    let (mut sender, mut receiver) = client.split();

    println!("Getting messages.");
    loop
    {
        let message = receiver.get_message().unwrap();
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
                        sender.send_message(channel, &out_str).unwrap();
                    }
                }
            },
            _ => {}
        }
    }
}
