from commands_core import slash_command, client

@slash_command 
def my_command(args): # /your_command_name {reqired} [optional] [optional]
    client.print("Hello World!") 