from commands_core import slash_command, client
# from commands_core import command_tools as ct

@slash_command 
def my_command(args):

    __ARGUMENTS__ = ["Argument 1", "Argument 2", "Argument 3"]

    client.print("Hello World!") 