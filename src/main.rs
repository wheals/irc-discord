extern crate discord;
extern crate irc;

use discord::Discord;
use discord::model::Event;

use irc::client::prelude::*;

use std::env;

fn main() {
    /*
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"),
    ).expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                if message.content == "!test" {
                    let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
                } else if message.content == "!quit" {
                    println!("Quitting.");
                    break
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
    */
    let server = IrcServer::new("config.json").unwrap();
    server.identify().expect("not to fail");
    for message in server.iter() {
        let msg = message.expect("a message");
        let src = msg.source_nickname().unwrap_or("a ghost");
        match msg.command {
            Command::PRIVMSG(_, ref text) => println!("<{}> {}", src, text),
            _ => ()
        }
    }
}
