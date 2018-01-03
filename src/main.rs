extern crate discord;
extern crate irc;

use discord::Discord;
use discord::model::Event;
use discord::model::ChannelId;

use irc::client::prelude::*;

use std::env;
use std::thread;
use std::collections::HashMap;
use std::str::FromStr;

fn discord_loop(mut connection: discord::Connection, server: IrcServer, chanmap: HashMap<String, String>) {
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(ref message)) if !message.author.bot => {
                let discord_channel = message.channel_id.to_string();
                if let Some((irc, _)) = chanmap.iter().find(|&(_, disc)| &discord_channel == disc) {
                    server.send_privmsg(irc, &message.content).unwrap();
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                ::std::process::exit(code.unwrap_or(0) as i32)
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
}

fn irc_loop(server: IrcServer, discord: Discord, chanmap: HashMap<String, String>) {
    for message in server.iter() {
        let msg = message.expect("a message");
        let src = msg.source_nickname().unwrap_or("a ghost");
        match msg.command {
            Command::PRIVMSG(ref target, ref text) => {
                let to_send = format!("<{}> {}", src, text);
                if let Some(discord_chan) = chanmap.get(target) {
                    let _ = discord.send_message(ChannelId(u64::from_str(discord_chan).expect("invalid channel ID")),
                                                 &to_send, "", false);
                } else {
                    println!("no output discord channel specified for message {}", to_send);
                }
            },
            _ => {
                println!("{}", msg.to_string());
            }
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
    let chanmap = config.options.clone().unwrap();
    let chanmap_ = config.options.clone().unwrap();
    let rcv_server = IrcServer::from_config(config).unwrap();
    rcv_server.identify().expect("IRC auth failed");
    let send_server = rcv_server.clone();

    let guard = thread::spawn(|| discord_loop(connection, send_server, chanmap));
    thread::spawn(|| irc_loop(rcv_server, discord, chanmap_));
    guard.join().expect("no panics");
}
