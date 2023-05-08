import gc as garbage_collector
import time
import uuid as guid

import inspect

"""
commands_header is a python file that contains functions required or useful
or writing slash commands for codename bonfire.

Alternate names: 
commands.h.py
"""

# This file contains botting functions you will likely need!

# def COLLECT_GARBAGE(): 
#     """A lazy persons way to write unsafe python"""

#     garbage_collector.collect()

def slash_command(function):
    """
    A nice little command wraper that serves both practical and semantic 
    pourposes. This tells the Client that this is a slash command, and
    also handles flags, user input, and all the repettive/heavy-lfting
    things you will see for almost every command.
    """

    def wrapper(*inputs):
        # We want to time the whole thing, so we need to reccord the start 
        # time. If they don't want a speed test, we can simply free this 
        # later.
        start_time = time.time_ns()

        # ENTER UNSAFE PYTHON
        # garbage_collector.disable() # <- only disables AUTOMATIC garbage collection. 
        # Can till be manually collected with garbage_collector.collect()

        # __init__ 
        client = Client()

        client.show_debug = True

        client.debug(f"Raw Input: {inputs}")

        inputs = inputs[0].split(" ")
        inputs.pop(0)

        client.debug(f"Split Input: {inputs}")

        flags = handle_flags(inputs, client=client)

        # - [ ] Combine Strings (IF DESIRED BY METADATA VARIABLE 
        #       (__COMBINE_STRINGS__ = True : Combine them! default: False))
        # - [ ] identify types
        #     - [ ] check for valid types 
        #         - [ ] Int: 0-9, replacng "_", " ", ",", "." with "" or "_" to make 
        #           it code readable
        #         - [ ] Float: 0-9, ",", "." -- replace them both is a "". If multple
        #               exist:
        #               - [ ] Replace the more common one with "".
        #               - [ ] If theres exactly one of each, whicever one comes last 
        #                    (right most), becomes the decmal, and the other becomes ""
        #               - [ ] If more than one of both "," and "." exists: throw an error
        #             and say you could not identify the decimal point. 


        # - [ ] convert to correct types (if none specified: String)


        # - [ ] Identify optional (_) paramaters
        #       - [ ] If not found, fill those with a null (None) value.

        # This is a mess. The condition is basically: 
        # "If the command takes morew arguments than we recieved as input, run this block"
        if len(inputs) <= (function_arguments := len(inspect.signature(function).parameters) - 3):
            #              ^^^^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^
            #              | Take the number of    | | Get function details    | |        |
            #              | arguments - 3 [2] and |                             | [1]    |
            #              | assign it[3] for later|   
            #              | use in the error msg  |
            #              
            # [1]: From the details, take the paramaters minus 3 (required paramaters that the user dosen't input)
            # [2]: 3 is the number of required arguments NOT supplied by the user necesarly. They are, in order, 
            #      *string_collector (can be blank), 
            #      client (supplied by wrapper), 
            #      **flags (supplied by wrapper, can be empty)

            client.error(f"Not enough arguments passed! Expected at least {function_arguments} arguments.")
            del function_arguments
            return

        if client.show_debug: client.devider()

        # We do this here so one variable can control both the wrappers debug
        # and the commands debug seperately. 
        client.show_debug = flags['debug']


        client.debug(f"Passing arguments: {inputs}")

        if flags["speed"] == True or flags["debug"] == True: 
            client.debug("Running with speed tests")

            function(*inputs, client=client, **flags)

            run_time = time.time_ns() - start_time

            if run_time >= 10_000: 
                client.info(f"This command took {round(run_time / 1_000_000, 2)}ms to complete!")
            elif run_time == 0: 
                client.warn("The speed test flag seems to be bugged, since it returned 0 nanosecconds.")
            elif run_time >= 10_000: 
                client.debug(run_time)
            elif (time.time_ns() - start_time) < 0:
                client.warn("The speed test function seems to be bugged, since it returned a negative value.")
            else: 
                client.info(f"This command took {time.time_ns() - start_time} NANOSECCONDS to complete!")
            
            del start_time, run_time
        else: 
            del start_time
            function(*inputs, client=client, **flags)
        
        del inputs, flags
        garbage_collector.collect() 
    return wrapper

def handle_flags(inputs, *, client):

    # - [x] identify
    # - [x] create
    # - [x] put into kwargs
    # - [ ] Cannot be in or after when a string is needed *UNLESS the 
    #       String is "quoted"
    # - [ ] must be after positional arguments, before a  multi-word string
    #       paramater
    # - [ ]Identify quoted strings and remove quotes

    flag_indexes = []
    flags = {
        "speed": False,
        "debug": False,
    }

    # TODO: Keep flags out of string collectors.
    # TODO: Let them add non `True` values to the `flags` dictionary.
    # I hate this messy code but this is the only place it will ever be used so extraction hardly makes sense here. 
    for index in range(len(inputs)):
        if not inputs[index].startswith("-"): continue

        if inputs[index].startswith("--"): 
            client.debug(f"Found flag: {inputs[index]}")

            flag_indexes.append(index)

            if "=" in inputs[index]: 
                client.debug("Found value flag")
                flag = inputs[index].split("=")

                client.debug(f"{flag}")

                key = str(flag[0]).lstrip("--")
                value = flag[1]
                del flag

                # Do float first to avoid rounding!
                try: value = float(value)
                except: client.debug("Value can be represented as a float")
                try: value = int(value)
                except: client.debug("Value can be represented as a intiger")

                flags[key] = value

                del key, value

                continue

            client.debug("Appending Flag to dictionary")
            flags[str(inputs[index].lstrip("-"))] = True

            flags[str(inputs[index].lstrip("-"))] = True
            client.debug(f"flags: {flags}")
            continue
        
        # else: 
        client.debug(f"Found short-form flags")

        flag_collector = inputs[index]
        for character in flag_collector:
            client.debug(f"Found flag: {character}")
            client.debug("Appending Flag to dictionary")
            flags[character] = True
    
    client.debug("Finished scanning")

    # del index

    client.debug("Starting flag popping")

    popped = 0
    for flag in flag_indexes: 
        inputs.pop(flag - popped)
        popped += 1

    # del popped, flag_indexes

    client.debug("Done flag popping")
    client.debug(f"Flag Processed Input: {inputs}")

    client.debug(f"Flags: {flags}")
    return flags

class Client: 
    """
    Interact with the users Client. This is the main class you will interact with. 
    """

    def __init__(self): 
        self.show_debug = True
        self.show_info = True
        self.show_warn = True
        self.show_error = True

        prefrences = {}

        # Set in the users client config
        # Do not share. 

        CLIENT_TOKEN = ""
        # PRIVATE_KEY = ""
        PUBLIC_KEY = ""

    # Print outputs
    # TODO: Add formatting so that: 
    #   - they will auto wrap after 80 characters, splitting at spaces
    #   - newlines always start with enough spaces to line it up with the inital text start.
    #
    # Desired output: https://cdn.discordapp.com/attachments/911129965972553729/1097997773816746118/image.png
    # from text: 
    # Client.info("""
    # Thank you for your interest! Some major wrapper features that are not implemented yet are:
    # - Single Letter Flags
    # - Type-checking
    # - Type Converters
    # - Client Interactions
    # - Auto-formatting of lient print functions for terminal use
    # 
    # Probably more! If you search for `TODO:` in our code base you will see many small enhancements!
    # """)
    def announce(self, string: str): 
        """Print a bot message for all* to see! * = sent as if it was a user account but with a bot tag."""
        print(f"\n\033[0m\033[90m Anounce:{command_tools.white} {string}", end="")
    
    def print(self, string: str): # Print a bot message/command response
        """Print a bot message that only the commands runner can see."""
        print(f"\n\033[0m\033[90m   Print:{command_tools.white} {string}", end="")

    def fatal(self, string: str, *, type="", command=""): 
        """Print an fatal error message for your bot. """
        print(f"\n   \033[0m\033[41m\033[30mFatal: {string}{command_tools.white}", end="")
    
    def error(self, string: str, *, type="", command=""): 
        """Print an error message for your bot. """

        if self.show_error: 
            print(f"\n\033[0m\033[91m   Error:{command_tools.white} {string}", end="")

    def warn(self, string: str, *, type="", command=""): 
        """Print an warning for your bot. Will only show up in the terminal
        and when your debug level is set to warn or higher."""

        if self.show_warn: 
            print(f"\n\033[0m\033[93m Warning:{command_tools.white} {string}", end="")

    def info(self, string: str, *, type="", command=""): 
        """Print debug info as your bot. Will only show up in the terminal
        and when your debug level is set to info or higher."""

        if self.show_info: 
            print(f"\n{command_tools.blue}    Info:{command_tools.white} {string}", end="")

    def debug(self, string: str): 
        """Prints debug messages to the dev terminal. These are often used
        to show what the command is currently doing and give insight as to
        how it works. They will only be written when the --debug flag is
        set."""

        if self.show_debug: 
            print(f"\n\033[0m\033[92m   Debug:{command_tools.white} {string} ...", end="")
    
    def devider(self): 
        print(f"\n\033[0m\033[90m   " + "-" * 74, end="")


    def get_instance(user="@me"): 
        return "chat.instance.tld"

    def get_guid_4(self): 
        return guid.uuid4()

    # :/ no, this does nothing right now.
    def input(self, promt: str): 
        string = input(f" {command_tools.blue}? {command_tools.gray}{promt}{command_tools.blue}")
        return string

    def message(self): # Print a message as the bot
        pass

    def message_box_content(self, string: str): # Edits the message box content when enter is pressed. Used for commands like /shrug.
        print(f"""\n{command_tools.output} Changed message box content to
          \"{string}\"""", end="")
        pass

    def get_message_list(channel: guid): 
        pass

    def set_nick(self, string: str, **flags): 
        pass

    # *, paramater will eat ALL positional arguments, making them all 
    # need to be keyword only
    def CONNECT(self, *, edition="2023", server: str): 
        """
        Used to establishing connections. Does nothing and will not be 
        added here as bots have no need to do this.
        """
        pass

    def CREATE(self, *, edition="2023", type="message", language, author, adressed, time, content: str): 
        """
        An advanced function to send a CREATE request to the server. Create 
        requests create content on the server such as a message, oreaction,
        vote, etc.
        """
        pass

    def NOTIFY(self, packet: str):
        """
        An advanced function to send a NOTIFY request to the server. Notify 
        requests notify or communicate with the Client or server, but for 
        one-time  packets that do not need to be stored: such as voice data, 
        establishing a one-time connection, etc.
        """
        pass

    def EDIT(self, *, edition="2023", type="message", target, time, content: str): 
        """
        An advanced function to send a EDIT request to the server. Edit 
        requests edit existing contnt on the server: think of editing a
        message or wiki page that already exists.
        """
        pass

    def REMOVE(self, *, edition="2023", target, conent=""): 
        """
        An advanced function to send a REMOVE request to the server. Remove
        requests remove the content from anyone but the owners view, but 
        does not delete any data. Useful if you want to possibly restore
        something later.

        Remove is a shorthand for editing the privacy in many, but not all, 
        cases. In cases where the content cannot be removed, (e.g., statuses, 
        bios, etc), it will instead reset it to its default vallue.
        """
        pass

    def DELETE(self, *, edition="2023", target): 
        """
        An advanced function to send a DELETE request to the server. Delete
        requests permently delete content from the server. 
        """
        pass

    def DESTORY(self, *, edition="2023", target): 
        """
        An advanced function to send a DESTROY request to the server. Destroy
        requests don't just delete the content, they also whipe over it with
        all 0's (in binary) making the data almost entirely unreocverable,
        even with physical access to the server. 
        
        Please think carefully if you really need this kind of request, as your 
        data will likely be written over soon anyway, and this degrades server 
        architecture much faster than it otherwise would. Not all instances
        allow sending DESTROY requests and will instead change them to DELETE.
        """
        pass

    def GET(self, *, edition="2023", type, range):
        """
        An advanced function to send a GET request to the server. Get requests
        retrieve data and return it to you. You can specify what paramaters you
        want to receive. If none are specified, it returns the entire message.
        """
        pass

    def REQUEST(self, *, edition="2023", **keyword_args):
        """
        An advanced function to send ANY type of request. Thos function adds
        no data header, letting you create it on your own entirely. 
        """
        # Packet send function with no defined operation.
        # This operation adds no data header, meaning you need to add it
        # yourself. This is often used with operation REQUEST for things
        # like server pings.
        pass 

#// Types get capital letters and are one word.

class Server: 
    def __init__(self): 
        pass
    
    def connect(self, server): 
        raise NotImplementedError

class Profile: 
    """
    Profile objects.
    """

    def __init__(self, identifier): 
        self.name = "test"
        self.guid16 = guid.uuid4()
        self.guid32_dyslexic = "GUID_32_dyslexic"
        self.display_name = identifier
        self.account = guid.uuid4()
        self.account_auth = guid.uuid4() # Let's pretend this is a token. It's NOT! But lets play pretend!

class User: 
    """
    What do you think a user is? Do you REALLY need help with this?
    """
    def __init__(self, identifier): 
        """
        Initiate a new user object. The identifier can be a guid16, a
        guid32_dyslexic, a handle (STARTING with the @ sign). All other
        variables needed will be retrieved via querys. 
        """

        self.display_name = "[FIELD IS DEPRICATED! Use the profiles name instead!]"
        self.login_name = "test"
        self.email = "user@email.email"
        self.guid16 = guid.uuid4()
        self.guid32_dyslexic = "GUID_32_dyslexic"
        self.profile = "uwu"
        # self.profiles = {"m": Profile(), "t": Profile(), "au": Profile()}


class command_tools: 
    gray = "\033[0m\033[90m"
    white = "\033[0m\033[97m"
    blue = "\033[0m\033[96m"
    output = f"{gray}  Output:{white}"

    def args_to_string(args): # Strip is number removed from the start
        string = ""

        for i in range(len(args)): 
            string += args[i] + " "
        
        return string.lstrip(" ").strip(" ")

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
            Client.print("GUID is not a valid base. Please use a base 16, 32, 64, or 10 GUID, either colon or hyphen seperated.")

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



import functools

def alias(alias_function):
    def _(_):
        @functools.wraps(alias_function)
        def _(*args, **kwargs):
            return alias_function(*args, **kwargs)
        return _
    return _
