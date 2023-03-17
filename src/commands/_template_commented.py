from commands_core import slash_command, client

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
def my_command(args): # /your_command_name {reqired} [optional] [optional]
    client.print("Hello World!") # Print a bot message

    # In most IDE's, including the simple one we have with our official client, 
    # you can type "client." to see possible output operations. You use these
    # to run actions based on the commands input.

    # Bot scripts are generally written in python. It was chosen becayuse of its
    # simple and easy to learn syntax. You can view the docs here: 
    # HTTPS://LINK-TO/PYTHON-TUTORIAL-FOR-ABSOLUTE-BEGINNERS

    # Keep in mind: arguments start at 0!

    # below is a basic program, try uncommenting it and look at its simple logic
    # to get started! Try uncommenting it, saving, and running /my_command
    # To uncomment multiple lines of code you can remove the '#'s manually or 
    # select it and press control + / 

    # if args[0] == "true": # Remember: unless you type convert them commands take strings, not other types!
    #     client.print("Dogs are cute!")
    # elif args[0] == "null": 
    #     client.print("Please fill in paramater 1!")
    # else: 
    #     client.print("Do you think dogs are cute?")

# Any client.CALL_CAPS functions are *ADVANCED* and require knowlege of the protocol
# amoung other things, and can be very dangorus if played with when you don't know
# what your doing!