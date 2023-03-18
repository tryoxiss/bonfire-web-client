from commands_core import slash_command, client
from commands_core import command_tools as ct

# This file handles commands that append a text emoticon to a message

def message_append(message, kaimoji): 
    message = ct.args_to_string(0, message)
    message += f" {kaimoji}"

@slash_command
def km_blush(args): 
    message = message_append(args, '(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄)')
    client.message_box_content(message)

# @slash_command
# def km_blush(args): 
#     message = message_append(args, '(⁄ ⁄>⁄ ▽ ⁄<⁄ ⁄)')
#     client.message_box_content(message)