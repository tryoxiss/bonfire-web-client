import uuid as guid

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
        # print(args)
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
        print(f"{command_tools.gray}Changed message content to:{command_tools.white} " + string)
        pass

    def get_message_list(channel: int): 
        pass

    def set_nick(string: str,  user: int): 
        pass


    def CREATE(**packet): 
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
        # i.e. you intend to display the requested content to the end user or 
        # process it for the end user.
        pass

    def REQUEST(packet: str): 
        # Packet send function with no defined operation.
        # This operation adds no data header, meaning you need to add it
        # yourself. This is often used with operation REQUEST for things
        # like server pings.
        pass 

class command_tools: 
    gray = "\033[90m"
    white = "\033[37m"

    def args_to_string(lstrip: int, args): # Strip is number removed from the start
        string = args

        for i in range(lstrip):
            string.pop(0)

        string = ' '.join(string)

        return string
    
    def process_user(identifer: str, **request) -> dict: 
        # identifier can be either a GUID or an @username.
        if identifer.startswith("@"): MODE = "HANDLE"
        else:                         MODE = "GUID"


        profile = { 
            'guid': 0,
            'handle': '@username',
            'instane': 'instace.tld',
            'display_name': "Display Name",
        }

        return profile
    
    def handle_guid(guid: str, base: int) -> int: 
        binary_guid = []

        if base == 32: 
            pass
        elif base == 16:
            pass
        elif base == 10:
            pass
        elif base == 64:
            pass
        else: 
            client.print("GUID is not a valid base. Please use a base 16, 32, 64, or 10 GUID, either colon or hyphen seperated.")

    # https://stackoverflow.com/questions/26929227/base-converter-in-python
    # def base_convert(base_to: str, number: int):
    #     if base_to == "32-obvious":
    #         digits = {c: i for i, c in enumerate('0123456789abcdefghijklmnopqrstuv')}
    #     if base_to == "32-dyslexic":
    #         digits = {c: i for i, c in enumerate('0123456788abcdefghijkmnpqrstuvwx')}
    #     if base_to == "16":
    #         digits = {c: i for i, c in enumerate('0123456789abcdef')}

    #     return sum(digits[digit] * (base_to ** i)
    #             for i, digit in enumerate(reversed(str(number))))


    # https://stackoverflow.com/questions/28824874/pythonic-way-to-do-base-conversion
    def base_convert(n, from_base=10, to_base=10):
        match to_base:
            case 16:
                convert_string = "0123456789ABCDEF"
            case "32-obvious":
                convert_string = "0123456789abcdefghijlmnopqrstuvw"
            case "32-dyslexic":
                convert_string = "0123456789abcdefghijkmnpqrstuvwx"
        
        return n