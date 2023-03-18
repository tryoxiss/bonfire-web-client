from commands_core import slash_command, client
from commands_core import command_tools as ct


@slash_command
def ban(args): # /ban {user} [duration] [reason] -- UNFINISHED

    __ARGUMENTS__ = ["User (@mention or guid)", "Duration (One Unit Only)", "Reason"]

    user = args[0]
    expires = args[1]
    reason = ""
    reason = ct.args_to_string(2, args)

    if ("f" or "_" or "*" or "p" or "perm" or "permenet" or "forv" or "forever" or "never") in args[1]: 
        expires = "never"
    elif "y" in args[1]: 
        pass
    elif "w" in args[1]: 
        pass
    elif "d" in args[1]: 
        pass
    elif "h" in args[1]: 
        pass
    else: 
        client.print("Invalid unit. Valid units are `y` (years), `w` (weeks), `d` (days), and `h` (hours). Additionally you can provide `f`, `_`, `*`, or `p` to ban them permenetly until revoked manually.")

    if expires == "never": 
        client.print(f"Banned {user} for {reason}")
    else: 
        client.print(f"Banned {user} for {reason}. This ban will expire in {expires}")

ban("/ban @username 3d this meowing user fucked up some shit.")

@slash_command
def shrug(args): 
    message = ct.args_to_string(0, args)

    message += " ¯\_(ツ)_/¯"

    client.message_box_content(message)