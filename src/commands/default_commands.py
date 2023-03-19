from commands_core import slash_command, client
from commands_core import command_tools as ct
import re as regex

IS_DEBUG_MODE = True

@slash_command
def ban(args): # /ban {user} [duration] [reason] -- UNFINISHED

    __ARGUMENTS__ = ["User (@mention or guid)", "Duration (One Unit Only)", "Reason"]

    user = args[0]
    expires = args[1]
    reason = ""
    reason = ct.args_to_string(2, args)

    if regex.search("[0-9]+[y, d, w, m]", expires.lower()):
        expires = expires.lower()

        # Now get the time in days, as the server-side ban engne 
        # wants it in DAYS only. Yes, this creates some bugs. For example
        # 12 months is 360 days, not 365 like you might expect.
        # But its close enough for real world use cases.
        if   "y" in expires:  # Year
            time = int(expires.rstrip("y") * 365)
        elif "m" in expires: # Month
            time = int(expires.rstrip("m") * 30)
        elif "w" in expires: # Week
            time = int(expires.rstrip("w") * 7)
        elif "d" in expires: # Day
            time = int(expires.rstrip("d"))     
        expires = time
    else: 
        reason = expires + " " + reason
        expires = "never"

    if expires == "never": 
        client.print(f"Banned {user} for \"{reason}\"")
    else: 
        client.print(f"Banned {user} for \"{reason}\". This ban will expire in {expires}")

# ban("/ban @username 3d this meowing user fucked up some shit.")

@slash_command
def shrug(args): 
    message = ct.args_to_string(0, args)

    message += " ¯\_(ツ)_/¯"

    client.message_box_content(message)

@slash_command
def whois(args): 
    pass


#// EMOTE BLOCK
@slash_command
def e(args): 
    message = ct.args_to_string(0, args)
    emote(f"/emote {message}")

@slash_command
def emote(args): 
    EMOTE = args[0]
    kaomoji = ""

    message = ct.args_to_string(1, args)

    match EMOTE: 
        case "sob":
            kaomoji = "(╥﹏╥)"
        case "blush": 
            kaomoji = "(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄)"
        case "shy": 
            kaomoji = "(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄)"
        case "greet":
            kaomoji = "(*・ω・)ﾉ"
        case "sorry": 
            kaomoji = "(シ_ _)シ"
        case "hug": 
            kaomoji = "(つ≧▽≦)つ"
        case "sleep": 
            kaomoji = "(－ω－) zzZ"
        case "spell": 
            kaomoji = "(ﾉ>ω<)ﾉ :｡･:*:･ﾟ’★,｡･:*:･ﾟ’☆"
        case "magic": 
            kaomoji = "(ﾉ>ω<)ﾉ :｡･:*:･ﾟ’★,｡･:*:･ﾟ’☆"

    # IDEAS: Greet, Love, Happy, Etc. 
    # List: http://kaomoji.ru/en/

    emote = f"{kaomoji}"

    message += f" {emote}"

    client.message_box_content(message)




while IS_DEBUG_MODE:
    # WHITE: \033[37m
    user_input = input(f"{ct.gray}Input: {ct.white}")
    if user_input.startswith("/"): 
        command_func = user_input.lstrip("/").split(" ")
        globals()[command_func[0]](user_input)

    elif user_input.startswith("exit") or user_input.startswith("quit"):
        exit(1)
    else:
        print(user_input)