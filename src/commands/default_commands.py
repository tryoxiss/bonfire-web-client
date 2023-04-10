from commands_header import slash_command, client, alias
from commands_header import command_tools as ct
import re as regex
from datetime import timedelta, datetime

IS_DEBUG_MODE = True

@slash_command
def nick(*nickname, **flags): 
    nickname = ct.args_to_string(nickname)

    if "user" in flags: 
        client.info("We currently only handle nicknaming yoursself in this script.")
    else: 
        nickname = f"{nickname}"
        client.set_nick(nickname)

@slash_command
def kick(user: str, *reason, **flags): 
    client.REMOVE(edition="2023", 
                  type="kick", 
                  target="GUID", 
                  content=reason)

# _duration: str = ""
# THIS IS A TERRIBLE WAY OF DOING THIS! I want to allow for bans with both 
# no duration and no message, and adding the default empty string is the
# only real way to do that :/

@slash_command
def ban(user: str, _duration: str = "", *reason, **flags): # _ means OPTIONAL __NOT__ UNUSED
    __ARGUMENTS__ = ["@mention or guid", "Duration (optional)", "Reason"]
    
    DEBUG = True # Set this based on a flag. Check with if DEBUG: client.info("String ...").

    if "--debug" in flags: 
        DEBUG = True
        client.info("Debug flag detected: Switching to debug mode!")

    expires = _duration

    reason = ct.args_to_string(reason)

    if DEBUG: client.debug("Scanning time field")

    if regex.search("[0-9]+[y, d, w, m]", expires.lower()):
        expires = expires.lower()

        if DEBUG: client.debug("Found time field")

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
        
        reason = reason
        if DEBUG: client.debug("Setting reason")
    else: 
        if DEBUG: client.debug("No time found")
        reason = expires + " " + reason
        expires = "never"
        if DEBUG: client.debug("TIme set to indefinte")

    if DEBUG: client.debug("Cleaning reason field")
    reason = reason.rstrip(" ").lstrip(" ")

    client.info("No ban object is currently sent with the /ban command.")

    # if DEBUG: client.info("Sending packet ...")

    if expires == "never": 
        client.print(f"Banned {user} for \"{reason}\" indefintely.")
    else: 
        lifted_on = datetime.today() + timedelta(days=expires, hours=1)
        client.print(f"Banned {user} for \"{reason}\". This ban will be lifed on {lifted_on.strftime('%d/%b/%Y at %H:00')}.")


#// EMOTE BLOCK (NOT WORKING)

@slash_command
def emote(EMOTE, *message, **flags):
    __ARGUMENTS__ = ["Emote", "Message"]

    kaomoji = ""
    message = ct.args_to_string(message)

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

    if EMOTE == "--list": 
        client.warn("This is not designed to be visually appealing yet! This is just the raw code!")
        client.print(index)
        return

    try: 
        message += f" {index[EMOTE]}"
    except: 
        client.print("So such kaomoji \"EMOTE\" found. You can add it in settings. Use the --list or -l flag to see all kaomoji in your list!")
    
    client.message_box_content(message)


@alias(emote)
def e(*message): pass # Can be anything not necesarly *message

@alias(emote)
def kao(*message): pass

@alias(emote)
def kaomoji(*message): pass

@alias(ban)
def getoffmylawn(*message): pass

###############################################################################
# BEGIN DEBUG BLOCK                                                           #
# --------------------------------------------------------------------------- #
# This cant be in the header, but copy+paste this into your file to have a    #
# command line test interface. The dev enviornment in our official client     #
# automatically injects this code at the end on program run.                  #
###############################################################################

while IS_DEBUG_MODE == True:
    user_input = input(f"{ct.gray}  Input: {ct.white}")
    if user_input.startswith("/"): 
        try: 
            command_func = user_input.lstrip("/").split(" ")
            globals()[command_func[0]](user_input)
            # List of exceptions: https://www.programiz.com/python-programming/exceptions
        except SyntaxError:
            client.error("The command had an inernal syntax error.")
        except KeyboardInterrupt: 
            client.info("Process Stopped (KeyboardInterrupt)")
        except MemoryError:
            client.error("You do not have enough memory to run this command.")
        except: 
            client.error(f"Either there is such command \"{user_input}\" or the command had an error it could not handle.")
    elif user_input.startswith("exit") or user_input.startswith("quit"):
        exit(1)