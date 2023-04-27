from commands_header import slash_command

@slash_command 
def my_command(arg1, arg2, *string_collector, client, **flags):

    __ARGUMENTS__ = ["Argument 1", "Argument 2", "Argument 3"]

    client.print("Hello World!") 