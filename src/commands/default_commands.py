from commands_header import slash_command, Client, alias, User
from commands_header import command_tools as ct
import re as regex
from datetime import timedelta, datetime

__AUTHORS__ = {
    "tryoxiss": {
        "Contact": "No",
        "Username": "tryoxiss",
        "real name": "no"
    }
}

__REPOSITORY__ = "https://github.com/tryoxiss/bonfire-server/"

@slash_command
def nick(*nickname, client, **flags) -> None: 
    nickname = ct.args_to_string(nickname)

    if "user" in flags: 
        client.info("We currently only handle nicknaming yoursself in this script.")
    else: 
        nickname = f"{nickname}"
        client.set_nick(nickname)

@slash_command
def kick(user: str, *reason, **flags) -> None: 
    client.REMOVE(edition="2023", 
                  type="kick", 
                  target="GUID", 
                  content=reason)

# _duration: str = ""
# THIS IS A TERRIBLE WAY OF DOING THIS! I want to allow for bans with both 
# no duration and no message, and adding the default empty string is the
# only real way to do that :/

@slash_command
def cmdebug(*, client, **flags):
    client.print("cuddle")

@slash_command
def ban(user: str, _duration: str = "", *reason, client, **flags) -> bool: # _ means OPTIONAL __NOT__ UNUSED
    __ARGUMENTS__ = ["@mention or guid", "Duration (optional)", "Reason"]

    # THIS IS A TERRIBLE WAY TO DO THIS!!! We want to just accept user as a User instnatly instead of needing to type convert!

    client.debug("Turning user string into User object")
    user = User(user)

    client.debug("If this prints a GUID, it is a user object.")
    client.debug(user.guid16)

    expires = _duration

    reason = ct.args_to_string(reason)

    client.debug("Parsing time field")

    if regex.search("[0-9]+[y, d, w, m]", expires.lower()):
        expires = expires.lower()

        # Round up to the next closet hour, and checked for unbans every hour.
        # So if you ban someone at 21:15 or 21:59 on January 4th for 1 day 
        # they would both be banned until 22:00 on January 5th that same year.
        # 
        # This is for performance and ease of math and use, as checking a lot
        # is quite frankly not worth the computing power when almost nobody is 
        # going to sit there for the minute thier ban is lifted.

        # Now get the time in days, as the server-side ban engne 
        # wants it in DAYS only. Yes, this creates some bugs. For example
        # 12 months is 360 days, not 365 like you might expect.
        # But its close enough for real world use cases.
        if   "y" in expires: expires = int(expires.rstrip("y")) * 365 # Year
        elif "m" in expires: expires = int(expires.rstrip("m")) * 30  # Month
        elif "w" in expires: expires = int(expires.rstrip("w")) * 7   # Week
        elif "d" in expires: expires = int(expires.rstrip("d"))       # Day

        client.debug("Setting reason")
        reason = reason
    else: 
        client.debug("Interpriting as: indefinte")
        reason = expires + " " + reason
        expires = "never"
        client.debug("Time set to indefinte")

    client.debug("Cleaning reason field")

    client.warn("No ban object is currently sent with the /ban command.")

    # if DEBUG: Client.info("Sending packet", DEBUG)

    if expires == "never": 
        client.print(f"Banned {user.display_name} for \"{reason}\" indefintely.")
    else: 
        lifted_on = datetime.today() + timedelta(days=expires, hours=1)
        client.print(f"Banned {user.display_name} for \"{reason}\". This ban will be lifed on {lifted_on.strftime('%d/%b/%Y at %H:00')}.")

@slash_command
def whois(user: str, *, client, **flags) -> None: 
    user = User(user)

    client.print(f"""
**Display Name:** {user.display_name}
**guid base 16:** `{user.guid16}`
    """)

#// EMOTE BLOCK (NOT WORKING)

@slash_command
def emote(EMOTE, *message, client, **flags) -> None:
    __ARGUMENTS__ = ["Emote", "Message"]

    client.debug("Init Kaomoji")
    kaomoji = ""
    client.debug("Parsing string")
    message = ct.args_to_string(message)

    client.debug("Setting kaomoji")
    index = { 
        # IDEAS: Greet, Love, Happy, Etc. 
        # List: http://kaomoji.ru/en/

        # "name": "replace with",
        "shrug": "¯\_(ツ)_/¯",
        "cast": "(ﾉ>ω<)ﾉ :｡･:*:･ﾟ’★,｡･:*:･ﾟ’☆",
        "spell": "(ﾉ>ω<)ﾉ :｡･:*:･ﾟ’★,｡･:*:･ﾟ’☆",
        "magic": "(ﾉ>ω<)ﾉ :｡･:*:･ﾟ’★,｡･:*:･ﾟ’☆",
        "cry": "(╥﹏╥)",
        "blush": "(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄)",
        "shy": "(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄)",
        "greet": "(*・ω・)ﾉ",
        "sorry": "(シ_ _)シ",
        "hug": "(つ≧▽≦)つ",
        "sleep": "(－ω－) zzZ",
        "joy": "(* ^ ω ^)",
        "sparkles-joy": "☆*:.｡.o(≧▽≦)o.｡.:*☆",
        "sparkles": "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧",
        "love": "(≧◡≦) ♡",
        "devious": "(・`ω´・)",
        "angry": "ヽ(`⌒´メ)ノ",
        "sob": ".･ﾟﾟ･(／ω＼)･ﾟﾟ･.",
        "hurt": "☆⌒(>。<)",
        "scared": "..・ヾ(。＞＜)シ",
        "wow": "w(°ｏ°)w",
        "wave": "ヾ(・ω・*)",
        "exited-hug": "(つ✧ω✧)つ",
        "nyoom": "─=≡Σ((( つ＞＜)つ",
        "sleep": "(－ω－) zzZ",
        "sing": "(￣▽￣)/♫•*¨*•.¸¸♪",
        "song": "(￣▽￣)/♫•*¨*•.¸¸♪",
        "facepalm": "(－‸ლ)",
        "tableflip": "( ╯°□°)╯ ┻━━┻",
        "unflip": "(╮°-°)╮┳━━┳",
        "innocent": "(◕‿◕✿)",
        "lazy": "_(:3 」∠)_",
    }

    if "list" in flags: 
        client.warn("This is not designed to be visually appealing yet! This is just the raw code!")
        client.print(index)
        return

    try: 
        message += f" {index[EMOTE]}"
    except: 
        client.print("So such kaomoji \"EMOTE\" found. You can add it in settings. Use the --list or -l flag to see all kaomoji in your list!")
    
    client.message_box_content(message)

@slash_command
def todo(*string, client, **flags): 
    client.info("""Thank you for your interest! Some major wrapper 
          features that are yet to be implemented are: 
          - Type-checking
          - Type Conversions
          - Client Interactions
          - Auto-formatting of Client print functions for terminal use

          Probably more! If you search for `TODO:` in our code base you will 
          many small enhancements!
    """)

@alias(emote)
def e(*message): pass # Can be anything not necesarly *message

@alias(emote)
def kao(*message): pass

@alias(emote)
def kaomoji(*message): pass

@alias(ban)
def getoffmylawn(*message): pass

@slash_command
def group(action, *, client, **flags): 
    
    if action.lower() == "new":
        instance = client.get_instance()

        name = client.input("Group Name: ")
        want_fancy_uri = client.input(f"""Do you want a fancy ID (y/n)? """)
        
        if want_fancy_uri == "y": 
            uri = client.input(f"""{ct.white}{instance}/group/""")
        elif want_fancy_uri == "n": 
            client.print("""You will get a GUID as your ID then. You can change
   this at any time.""")
        else: 
            client.info("Interpriting as: no")
            uri = client.get_guid_4()

        del want_fancy_uri

        client.print(f"""
        Your group links will look like this: 
   https://{instance}/group/{uri}/category/channel/
   ocp://{instance}/{uri}/category/channel/ <-- PROTOCOL NAME WIP

""")
        
        import time

        client.input(f"""Do you want a starter template (y/n): """)

        client.debug("""All these following messages are cosmedic! This command
   currently does nothing!""")
        client.debug("Allocating storage")
        time.sleep(0.1)

        client.debug("Preparing template")
        time.sleep(0.03)

        client.debug("Generating keys")
        time.sleep(0.2)

        client.debug("connecting")
        time.sleep(0.2)

        client.debug("joining")
        time.sleep(0.2)

###############################################################################
# BEGIN DEBUG BLOCK                                                           #
# --------------------------------------------------------------------------- #
# This cant be in the header, but copy+paste this into your file to have a    #
# command line test interface. The dev enviornment in our official Client     #
# automatically injects this code at the end on program run.                  #
###############################################################################

if IS_DEBUG_MODE := True: 
    import os
    from commands_header import Client

client = Client()

try: 
    while IS_DEBUG_MODE == True: 
        user_input = input(f"\n{ct.gray}   Input: {ct.white}")

        if not user_input.startswith("/"): continue

        try: 
            command_func = user_input.lstrip("/").split(" ")
            globals()[command_func[0]](user_input)
        # List of exceptions: https://www.programiz.com/python-programming/exceptions
        # https://docs.python.org/3/library/exceptions.html
        except SyntaxError:
            client.error("The command had an inernal syntax error.")
        except KeyboardInterrupt: 
            client.info("Process Stopped (KeyboardInterrupt)\n")
            exit(1)
        except MemoryError:
            client.error("You do not have enough memory to run this command... somehow? (these commands take like 100 bytes ._.)")
        except AttributeError:
            client.error("a `python:AttributeError` occured.")
        except NotImplementedError: 
            client.warn("A feature you are trying to use here is not implemented yet. ")
            client.info(f"""If you would like to help implement this yourself, please see our 
            repository at: {__REPOSITORY__}
            A list of yet-to-be-implemented features can be found by running 
            `/todo`!""")
        except LookupError: 
            client.error(f"No such command or internal function \"{command_func[0]}\" eixsts")
        # except: ImportError:
        #     Client.error("")
        except: 
            client.error("An unknown error occured. This may be in your block or the commands header.")
            client.info("This is most often")
            # Client.error(f"Either there is such command \"{user_input}\" or the command had an error it could not handle.")

        if user_input.startswith("exit") or user_input.startswith("quit"): 
            client.info("Process Stopped (ExitCommand)\n")
            exit(1)
except KeyboardInterrupt: 
    client.info("Process Stopped (KeyboardInterrupt)\n")
    exit(1)
except: 
    SLASH_COMMAND_IMPORTED = False
    CLIENT_IMPORTED = False
    ALIAS_IMPORTED = False
    COMMAND_TOOLS_IMPORTED = False

    errors = 0

    try: 
        client.error("The command line interface loop failed.")
    except: 
        print("""\033[0m\033[91m   Error:\033[0m\033[97m The Client class is not imported!
          If it IS imported, please make sure it is not renamed as that causes 
          this diagnostic to fail, leading to false positives.""")
        
        exit(0)

    try: 
        @slash_command
        def test_slash_imported_function(*arguments, **flags):
            client.debug("Test Function")
        test_slash_imported_function("argument")
    except: client.error("Slash command decorator not imported!")
    else: 
        client.info("""All required modules appear to be imported.
          If you got this error, that means you are likely using 
          an unimported module.

          Please check that you are not renaming any core modules 
          as that will cause this diagnostic to fail.
    """)
        client.info("Exiting ... ")