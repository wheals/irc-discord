This is a simple program for mirroring messages from IRC to Discord.

The goal is to pretend that you're on IRC when you actually check and send
messages from Discord. To that end, the messages coming from IRC state the
name of the sender, but not the ones going to IRC.

The config should look something like this:
```json
{
    "nickname": "wheals",
    "nick_password": "<password>",
    "server": "irc.freenode.net",
    "options": {
        "#wheals": "<discord channel id>",
        "##wheals": "<another discord channel id>",
        etc...
        "DISCORD_TOKEN": "<discord bot token>"
    }
}
```
