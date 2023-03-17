from commands_core import slash_command, client

@slash_command
def ban(args): # /ban {user} [duration] [duration]
    client.print("banned :D")

ban("/ban username 3d6h meow")