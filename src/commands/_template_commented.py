from commands_core import slash_command, client
# from commands_core import command_tools as ct # This is useful if you want to do more stuff

# Welcome to the command script template! We hope this helps ease you into botting
# or at least makes it a bit easier to get started!
#
# Below is the function name, it can be whatever you want it to be! 
# Keep in mind: the function name is case insensative, but otherwise is exactly
# what you type to run the command. In the example below you may type
# /your_command_name arg1 arg2 etc
# 
# Arguments are not part of the command, and instead are taken by the command
# function be be processed.

@slash_command # <-- This is important, as it does a lot of heavy lifting for you and tells your client to treat it as a slash command.
def my_command(arg1, arg2, *string_collector):
    # Define your arguments. This is how the end user knows what to put in each field. If this dosen't exist, we will instead show the variable names.
    __ARGUMENTS__ = ["Argument 1", "Argument 2", "Argument 3"] 

    client.print("Hello World!") # Print a bot message

    # In most IDE's, including the simple one we have with our official client, 
    # you can type "client." to see possible output operations. You use these
    # to run actions based on the commands input.

    # Bot scripts are generally written in python. It was chosen because of its
    # simple and easy to learn syntax. You can view a beginners tutorial here: 
    # HTTPS://LINK-TO/PYTHON-TUTORIAL-FOR-ABSOLUTE-BEGINNERS
    #
    # We also have a more specific guide on comman creation here: 
    # HTTPS://LINK-TO/BONFIRE-COMMAND-TUTORIAL

    # Keep in mind: arguments start at 0!

# Any client.CALL_CAPS functions are *ADVANCED* and require knowlege of the protocol
# amoung other things, and can be very dangorus if played with when you don't know
# what your doing!