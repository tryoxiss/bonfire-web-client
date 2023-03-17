# This file contains botting functions you will likely need!

def slash_command(function):
    def wrapper(message):
        # Mostly a cosmedic decorator to tell the rust
        # that the function is a slash command, and to
        # add it to the index.
        #
        # This also seperates the arguments into an
        # array and removes the inital command name
        # to make it easier to work with <3
        args = message.split(" ")
        args.pop(0)
        print(args)
        function(args)

    return wrapper



class client: 
    def __init__(): 
        print("No __init__ needed!")
    
    def print(string: str): # Print a bot message/command response
        print(string)

    def message(): # Print a message as the bot
        pass

    def message_box_content(string: str): # Edits the message box content when enter is pressed. Used for commands like /shrug.
        pass

    def get_message_list(channel: int): 
        pass

    def set_nick(string: str,  user: int): 
        pass

    def meow(n): 
        print("meow" * n)




    def CREATE(packet: str): 
        pass

    def EDIT(packet: str): 
        pass

    def REMOVE(packet: str): 
        pass

    def DELETE(packet: str): 
        pass

    def VIEW(packet: str): 
        pass