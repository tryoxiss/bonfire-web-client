from commands_core import slash_command

@slash_command
def ban(args): # /ban {user} [duration] [duration]
    # user = args[1].guid()

    guid = "GUID"
    signature = "SIGNATURE"

    duration = args[3]

    print(duration)

    # expires is the amount of time frtom the server
    # receving the packet that the server will add
    # the ban timer for. Never will make it permenet
    # until manually revoked.

    string = f"""
edition: 2022
type: ban
operation: CREATE

target: {guid}
expires: {duration} 

signature: {signature}
"""

ban("/ban username 3d6h meow")