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


    def CREATE(packet: str): 
        # Packet with the operation CREATE, 
        # i.e. you intend to create content on the server.
        pass

    def EDIT(packet: str): 
        # Packet with the operation EDIT
        # i.e. you intend to edit content on the server.
        pass

    def REMOVE(packet: str): 
        # Packet with the operation REMOVE
        # i.e. you intend to remove content on the server from anyones view
        # except your own. Useful when you may want to restore content.
        pass

    def DELETE(packet: str): 
        # Packet with the operation DELETE
        # i.e. you intend to permently delete content on the server
        pass

    def VIEW(packet: str):
        # Packet with the operation VIEW
        # i.e. you intend to display the requested content to the end user. 
        pass

    def REQUEST(packet: str): 
        # Packet with the operation REQUEST
        # i.e. you intend to only process the requested content on the backend.
        pass 