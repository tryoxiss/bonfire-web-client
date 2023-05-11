from commands_header import slash_command, Client, alias

from commands_header import command_tools as ct

__REPOSITORY__ = "github.com/tryoxiss/bonfire-server (temperary)"

@slash_command 
def switch(*front, client, **flags):

    __ARGUMENTS__ = ["Argument 1", "Argument 2", "Argument 3"]

    client.print("Hello World!") 




###############################################################################
# BEGIN DEBUG BLOCK                                                           #
# --------------------------------------------------------------------------- #
# This cant be in the header, but copy+paste this into your file to have a    #
# command line test interface. The dev enviornment in our official Client     #
# automatically injects this code at the end on program run.                  #
###############################################################################

if IS_DEBUG_MODE := True: 
    import os
    from commands_header import slash_command, Client, alias

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