extern crate discord;
extern crate irc;

use discord::Discord;
use discord::model::Event;

use irc::client::prelude::*;

use std::env;
use std::thread;

fn discord_loop(mut connection: discord::Connection, server: IrcServer, irc_channel: String) {
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                server.send_privmsg(&irc_channel,
                                    &message.content).unwrap();
                if message.content == "!quit" {
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
}

fn irc_loop(server: IrcServer) {
    for message in server.iter() {
        let msg = message.expect("a message");
        let src = msg.source_nickname().unwrap_or("a ghost");
        match msg.command {
            Command::PRIVMSG(_, ref text) => println!("<{}> {}", src, text),
            _ => ()
        }
    }
}

fn main() {
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"),
    ).expect("discord login failed");
    // Establish and use a websocket connection
    let (connection, _) = discord.connect().expect("connect failed");

    let config = Config::load("config.json").unwrap();
    let irc_channel = config.channels()[0].to_string();
    let rcv_server = IrcServer::from_config(config).unwrap();
    rcv_server.identify().expect("IRC auth failed");
    let send_server = rcv_server.clone();

    let guard = thread::spawn(|| discord_loop(connection, send_server, irc_channel));
    thread::spawn(|| irc_loop(rcv_server));
    guard.join().expect("no panics");
}
