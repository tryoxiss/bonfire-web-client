import uuid as guid
import time

"""
commands_header is a python file that contains functions required or useful
or writing slash commands for codename bonfire.

Alternate names: 
commands.h.py
"""

# This file contains botting functions you will likely need!

def slash_command(function):
    def wrapper(*args):
        # Mostly a cosmedic decorator to tell the rust that the function is a
        # slash command, and to add it to the index.
        #
        # This also deals with some argument backend that needs to be done for 
        # every command. 
        args = args[0].split(" ")
        args.pop(0)
            
        ### LOGIC: 
        # - Handle Flags (identify, create, and then put into kwargs)
        #   - Cannot be in or after when a string is needed *UNLESS the tring 
        #     is "quoted"
        #   - must be after positional arguments, before a  multi-word string
        #     paramater
        #   - Identify quoted strings and remove quotes
        # - Combine Strings (IF DESIRED BY METADATA VARIABLE 
        #   (__COMBINE_STRINGS__ = True : Combine them! default: False))
        # - identify types
        #   - check for valid types 
        #       - Int: 0-9, replacng "_", " ", ",", "." with "" or "_" to make 
        #         it code readable
        #       - Float: 0-9, ",", "." -- replace them both is a "". If multple
        #         exist:
        #           - Replace the more common one with "".
        #           - If theres exactly one of each, whicever one comes last 
        #             (right most), becomes the decmal, and the other becomes ""
        #           - If more than one of both "," and "." exists: throw an error
        #             and say you could not identify the decimal point. 
        #   - convert to correct types (if none specified: String)

        ## Also we want it to look at the paramaters needed and allow skipping of optional paramaters by providing null [MAYBE]

        # All caps flags are handled by this and NOT PASSED ON. So for example
        # --SPEED-TEST will also return the time the command took to run.

        # client.fatal("Test Fatal")
        # client.error("Test Error")
        # client.warn("Test Warn")
        # client.info("Test Info")
        # client.print("Test Print")

        # Please find a better way to name this. Or even better, don't need a variable just check in the if statement
        SPEED_TEST_FLAG = False

        if SPEED_TEST_FLAG == True: 
            start_time = time.time_ns()
            function(*args)
            # client.info(f"This command took {round((time.time_ns() - start_time) / 1_000_000, 2)}ms to complete!")
            # client.info(f"This command took {time.time_ns() - start_time} NANOSECCONDS to complete!")
        else: 
            function(*args)
    return wrapper

def speed_test(function):
    def wrapper(*args):

        try: 
            start_time = time.time_ns()
        except:
            client.error("Failed to get the time value.")
        function(*args)

        if (time.time_ns() - start_time) >= 10_000: 
            client.info(f"This command took {round((time.time_ns() - start_time) / 1_000_000, 2)}ms to complete!")
        elif (time.time_ns() - start_time) == 0:
            client.warn("The speed test function seems to be bugged, since it returned 0 nanosecconds.")
        elif (time.time_ns() - start_time) < 0:
            client.warn("The speed test function seems to be bugged, since it returned a negative value.")
        else: 
            client.info(f"This command took {time.time_ns() - start_time} NANOSECCONDS to complete!")
    return wrapper

class client: 
    def __init__(): 
        print("No __init__ needed!")

    def announce(string: str): 
        """Print a bot message for all* to see! * = sent as if it was a user account but with a bot tag."""
        print(f"\033[0m\033[90mAnounce:{command_tools.white} {string}")
    
    def print(string: str): # Print a bot message/command response
        """Print a bot message that only the commands runner can see."""
        print(f"\033[0m\033[90m  Print:{command_tools.white} {string}")

    def fatal(string: str, *, type="", command=""): 
        """Print an fatal error message for your bot. """
        print(f"  \033[0m\033[41m\033[97mFatal:{command_tools.white} \033[47m\033[30m{string}{command_tools.white}")
    
    def error(string: str, *, type="", command=""): 
        """Print an error message for your bot. """
        print(f"\033[0m\033[91m  Error:{command_tools.white} {string}")

    def warn(string: str, *, type="", command=""): 
        """Print an warning for your bot. Will only show up in the terminal
        and when your debug level is set to warn or higher."""
        print(f"\033[0m\033[93mWarning:{command_tools.white} {string}")

    def info(string: str, *, type="", command=""): 
        """Print debug info as your bot. Will only show up in the terminal
        and when your debug level is set to info."""
        print(f"\033[0m\033[96m   Info:{command_tools.white} {string}")
    

    # :/ no, this does nothing right now.
    def input(string: str): 
        string = input()
        return string

    def message(): # Print a message as the bot
        pass

    def message_box_content(string: str): # Edits the message box content when enter is pressed. Used for commands like /shrug.
        print(f"{command_tools.gray}Changed message content to:{command_tools.white} " + string)
        pass

    def get_message_list(channel: int): 
        pass

    def set_nick(string: str,  user: int): 
        pass


    def CONNECT(): 
        """
        Used to establishing connections. Does nothing and will not be 
        added here as bots have no need to do this.
        """
        pass

    def CREATE(**packet): 
        """
        An advanced function to send a CREATE request to the server. Create 
        requests create content on the server such as a message, oreaction,
        vote, etc.
        """
        pass

    def NOTIFY(packet: str):
        """
        An advanced function to send a NOTIFY request to the server. Notify 
        requests notify or communicate with the client or server, but for 
        one-time  packets that do not need to be stored: such as voice data, 
        establishing a one-time connection, etc.
        """
        pass

    def EDIT(packet: str): 
        """
        An advanced function to send a EDIT request to the server. Edit 
        requests edit existing contnt on the server: think of editing a
        message or wiki page that already exists.
        """
        pass

    def REMOVE(packet: str): 
        """
        An advanced function to send a REMOVE request to the server. Remove
        requests remove the content from anyone but the owners view, but 
        does not delete any data. Useful if you want to possibly restore
        something later.

        Remove is a shorthand for editing the privacy.
        """
        pass

    def DELETE(packet: str): 
        """
        An advanced function to send a DELETE request to the server. Delete
        requests permently delete content from the server. 
        """
        pass

    def DESTORY(packet: str): 
        """
        An advanced function to send a DESTROY request to the server. Destroy
        requests don't just delete the content, they also whipe over it with
        all 0's (in binary) making the data almost entirely unreocverable,
        even with physical access to the server. 
        
        Please think carefully if you really need this kind of request, as your 
        data will likely be written over soon anyway and this degrades server 
        architecture much faster than it otherwise would. Not all instances
        allow sending DESTROY requests and will instead change them to DELETE.
        """
        pass

    def GET(packet: str):
        """
        An advanced function to send a GET request to the server. Get requests
        retrieve data and return it to you. You can specify what paramaters you
        want to receive. If none are specified, it returns the entire message.
        """
        pass

    def REQUEST(packet: str):
        """
        An advanced function to send ANY type of request. Thos function adds
        no data header, letting you create it on your own entirely. 
        """
        # Packet send function with no defined operation.
        # This operation adds no data header, meaning you need to add it
        # yourself. This is often used with operation REQUEST for things
        # like server pings.
        pass 



### MIGHT REMOVE THE USER CLASS
class user: 
    """
    Hands user related actions.
    """
    def get_guid16(username): 
        pass

    def get_account(identifier): 
        """
        The identifier can be an @ or GUID16
        """
        pass



class command_tools: 
    gray = "\033[0m\033[90m"
    white = "\033[0m\033[97m"

    def args_to_string(args): # Strip is number removed from the start
        string = ""

        for i in range(len(args)): 
            string += args[i] + " "

        return string
    
    def process_user(identifer: str, **request) -> dict: 
        # identifier can be either a GUID or an @username.
        if identifer.startswith("@"): MODE = "HANDLE"
        else:                         MODE = "GUID"

        return { 
            'guid': 0,
            'handle': '@username',
            'instane': 'instace.tld',
            'display_name': "Display Name",
        }
    
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