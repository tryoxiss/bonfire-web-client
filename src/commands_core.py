def slash_command(func):
    def wrapper(message):
        # Mostly a cosmedic decorator to tell the rust
        # that the function is a slash command, and to
        # add it to the index.
        #
        # This also seperates the arguments into an
        # array and removes the inital command name
        # to make it easier to work with <3
        args = message.split(" ")
        args.pop(0)
        print(args)
        func(args)

    return wrapper