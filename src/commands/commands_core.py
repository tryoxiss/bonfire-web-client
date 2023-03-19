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
    
    def validate_guid(guid: str, base: int) -> int: 
        binary_guid = []

        binary_char_dyslexic = { 
            "0": b"00000",
            "1": b"00001",
            "2": b"00010",
            "3": b"00011",
            "4": b"00100",
            "5": b"00101",
            "6": b"00110",
            "7": b"00111",
            "8": b"01000",
            "9": b"01001",

            "a": b"01010",
            "b": b"01011",
            "c": b"01100",
            "d": b"01101",
            "e": b"01110",
            "f": b"01111",
            "g": b"10000",
            "h": b"10001",
            "j": b"10010",
            "k": b"10011",
            "m": b"10100",
            "n": b"10101",
            "p": b"10110",
            "q": b"10111",
            "r": b"11000",

            "s": b"11001",
            "t": b"11010",
            "u": b"11011",
            "v": b"11100",
            "w": b"11101",
            "x": b"11110",
            "y": b"11111",
        }

        if base == 32: 
            for i in len(guid): 
                binary_guid[i] = binary_char_dyslexic[guid[i]]
                pass
        elif base == 16:
            pass
        elif base == 10:
            pass
        elif base == 64:
            pass
        else: 
            client.print("GUID is not a valid base. Please use a base 16, 32, 64, or 10 GUID, either colon or hyphen seperated.")