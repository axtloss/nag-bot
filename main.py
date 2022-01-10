import discord
import os
import sys
import config
from discord.ext import commands
from notify_run import Notify
from datetime import datetime

class DevNull:
    def write(self, msg):
        pass

def log(foo):
    time = datetime.now().strftime('%Y/%m/%d %H:%M:%S')
    formatted = time + ' - ' + foo
    print(formatted)
    log_file = open('log.txt', 'a')
    log_file.write(formatted + '\n')

def bot_run():
    prefix = config.prefix
    name = config.name
    token = config.token

    bot = commands.Bot(command_prefix=prefix)
    notify = Notify()

    @bot.event
    async def on_ready():
        log('bot online, cwd = ' + os.getcwd())
        await bot.change_presence(activity=discord.Activity(type=discord.ActivityType.watching, name='for ,'), status=discord.Status.dnd)

    @bot.command()
    async def nag(foo, *, contents:str):
        """
        This command will nag the dev in the form of a Mobile and Desktop notification.
        """
        await foo.send('__' + name + '__ nagged \n__contents__: \"' + contents + '\"')
        notify.send(name + ': ' + contents)
        log('bot nagged ' + name + ' with contents: ' + contents)

    @bot.command()
    @commands.is_owner()
    async def shutdown(foo):
        """
        Shuts down the bot completely (owner only)
        """
        await foo.bot.change_presence(status=discord.Status.invisible)
        await foo.send('bot going down')
        await foo.bot.close()
        log('bot offline')

    bot.run(token)

if __name__ == '__main__':
    bot_run()
    if os.environ.get('NAGBOT_DEBUG') != 'yes':
        sys.stderr = DevNull() # comment out when debugging