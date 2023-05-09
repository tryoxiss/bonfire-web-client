from commands_header import slash_command
from commands_header import command_tools as ct
import random

@slash_command 
def tired(*, client, **flags) -> bool:

    gifs = [
        "https://tenor.com/view/anime-tired-anime-girl-cute-kawaii-gif-14818724",
        "https://tenor.com/view/bed-girl-tired-exhausted-anime-gif-12802615",
        "https://tenor.com/view/lucky-star-yawn-tired-sleepy-sleep-gif-8472935",
        "https://tenor.com/view/tired-bored-anime-girl-bed-gif-18074141",
        "https://tenor.com/view/tired-anime-girl-bed-gif-12806723",
        "https://tenor.com/view/anime-sleep-gif-19525636",
        "https://tenor.com/view/rin-shelter-anime-girl-anime-girl-gif-16378676",
        "https://tenor.com/view/willcore-kon-anime-girl-sleepy-gif-24035077",
        "https://tenor.com/view/anime-girl-re-zero-starting-life-in-another-world-emilia-gif-17724637",
        "https://tenor.com/view/ugh-yawn-tired-anime-sleepy-gif-17034115",
        "https://tenor.com/view/nogamenolife-shiro-sleepy-tired-exhausted-gif-6238156",
        "https://tenor.com/view/anime-tired-sleep-ranma-gif-24059343",
        "https://tenor.com/view/tired-yawn-gape-azur-lane-laffey-gif-20063007",
        "https://tenor.com/view/anime-tired-exhausted-gif-9595436",
        "https://tenor.com/view/anime-girl-tired-yawning-gif-12799669",
        "https://tenor.com/view/anime-tired-sleep-ranma-gif-24059343",
    ]

    client.message_box_content(f"{gifs[random.randrange(0, len(gifs))]}")


@slash_command 
def bored(*, client, **flags) -> bool:

    gifs = [
        "https://tenor.com/view/cute-bored-im-bored-panda-anime-gif-16619785",
        "https://tenor.com/view/music-ssss-dynazenon-anime-girl-gif-21583218",
        "https://tenor.com/view/bouuuuuuuuuuuuuuuuuuuuuuuuuu-bored-anime-cute-kawaii-gif-13095377",
        "https://tenor.com/view/bored-anime-bubbles-gif-14078318",
        "https://tenor.com/view/sleepy-anime-tired-bored-gif-12003953",
        "https://tenor.com/view/anime-tired-exhausted-gif-9595436",
    ]

    client.message_box_content(f"{gifs[random.randrange(0, len(gifs))]}")


@slash_command 
def cuddle(*, client, **flags) -> bool:

    gifs = [
        "https://tenor.com/view/kon-yui-azusa-yui-hirasawa-azusa-nakano-gif-18946600",
        "https://tenor.com/view/anime-hug-anime-anime-girl-anime-girls-anime-girls-hugging-gif-26094816",
        "https://tenor.com/view/hug-love-cuddle-sweet-gif-16300151",
    ]

    client.message_box_content(f"{gifs[random.randrange(0, len(gifs))]}")

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