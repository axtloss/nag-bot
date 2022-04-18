# Nag Bot
A bot to nag specific people if a ping won't work

## Install
```bash
git clone https://git.tar.black/axtlos/crystal-nag-bot.git
cd crystal-nag-bot
sudo make install
```
once you have installed you will need to edit the configuration (config.json) and save it to `~/.config/nagbot.json`

after that you can launch the bot by running `nag-bot` in the terminal

## Usage
Every person added in the config will need [https://ntfy.sh/](https://ntfy.sh/) installed and subscribed to their feed. Then everyone who has the role to nag people can use `\<activator> \<person> \<message>` to nag people, so with the example config it would be `!nag usr1 message`.