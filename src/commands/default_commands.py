from commands_header import slash_command, client
from commands_header import command_tools as ct
import re as regex
from datetime import timedelta, datetime

IS_DEBUG_MODE = True

# _duration: str = ""
# THIS IS A TERRIBLE WAY OF DOING THIS! I want to allow for bans with both 
# no duration and no message, and adding the default empty string is the
# only real way to do that :/

@slash_command
def ban(user: str, _duration: str = "", *reason): # _ means OPTIONAL __NOT__ UNUSED
    __ARGUMENTS__ = ["@mention or guid", "Duration (optional)", "Reason"]

    expires = _duration
    str_reason = ""

    str_reason = ct.args_to_string(reason)

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
        if   "y" in expires:  # Year
            expires = int(expires.rstrip("y")) * 365
        elif "m" in expires: # Month
            expires = int(expires.rstrip("m")) * 30
        elif "w" in expires: # Week
            expires = int(expires.rstrip("w")) * 7
        elif "d" in expires: # Day
            expires = int(expires.rstrip("d"))
        
        reason = str_reason
    else: 
        reason = expires + " " + str_reason
        expires = "never"

    reason = reason.rstrip(" ").lstrip(" ")

    if expires == "never": 
        client.print(f"Banned {user} for \"{reason}\" indefintely.")
    else: 
        lifted_on = datetime.today() + timedelta(days=expires, hours=1)
        client.print(f"Banned {user} for \"{reason}\". This ban will be lifed on {lifted_on.strftime('%d/%b/%Y at %H:00')}.")

    client.CREATE(content=reason, expires=expires)


#// EMOTE BLOCK (NOT WORKING)
@slash_command
def e(*message): 
    __ARGUMENTS__ = ["Emote", "Message"]
    message = ct.args_to_string(message)
    emote(f"/emote {message}")

@slash_command
def emote(EMOTE, *message): 
    __ARGUMENTS__ = ["Emote", "Message"]

    kaomoji = ""
    message = ct.args_to_string(message)

    match EMOTE: 
        case "shrug": 
            kaomoji = "¯\_(ツ)_/¯"
        case "cry":
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
        case "cast": 
            kaomoji = "(ﾉ>ω<)ﾉ :｡･:*:･ﾟ’★,｡･:*:･ﾟ’☆"
        case "joy": 
            kaomoji = "(* ^ ω ^)"
        case "sparkles-joy": 
            kaomoji = "☆*:.｡.o(≧▽≦)o.｡.:*☆"
        case "sparkles": 
            kaomoji = "(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧"
        case "love": 
            kaomoji = "(≧◡≦) ♡"
        case "devious": 
            kaomoji = "(・`ω´・)"
        case "angry": 
            kaomoji = "ヽ(`⌒´メ)ノ"
        case "sob": 
            kaomoji = ".･ﾟﾟ･(／ω＼)･ﾟﾟ･."
        case "hurt": 
            kaomoji = "☆⌒(>。<)"
        case "scared": 
            kaomoji = "..・ヾ(。＞＜)シ"
        case "wow": 
            kaomoji = "w(°ｏ°)w"
        case "wave": 
            kaomoji = "ヾ(・ω・*)"
        case "sparkle-hug": 
            kaomoji = "(つ✧ω✧)つ"
        case "nyoom": 
            kaomoji = "─=≡Σ((( つ＞＜)つ"
        case "sleep": 
            kaomoji = "(－ω－) zzZ"
        case "sing": 
            kaomoji = "(￣▽￣)/♫•*¨*•.¸¸♪"
        case "song": 
            kaomoji = "(￣▽￣)/♫•*¨*•.¸¸♪"
        case "facepalm": 
            kaomoji = "(－‸ლ)"
        case "tableflip": 
            kaomoji = " ( ╯°□°)╯ ┻━━┻"
        case "unflip": 
            kaomoji = "(╮°-°)╮┳━━┳"
        case "innocent": 
            kaomoji = "(◕‿◕✿)"
        case "lazy": 
            kaomoji = "_(:3 」∠)_"
        case _:
            client.print(f"No kaomoji \"{EMOTE}\" found.")

    # IDEAS: Greet, Love, Happy, Etc. 
    # List: http://kaomoji.ru/en/

    message += f" {kaomoji}"

    client.message_box_content(message)


###############################################################################
# BEGIN DEBUG BLOCK                                                           #
###############################################################################

while IS_DEBUG_MODE:
    user_input = input(f"{ct.gray}Input: {ct.white}")
    if user_input.startswith("/"): 
        command_func = user_input.lstrip("/").split(" ")
        globals()[command_func[0]](user_input)
    elif user_input.startswith("exit") or user_input.startswith("quit"):
        exit(1)
    else:
        print(user_input)