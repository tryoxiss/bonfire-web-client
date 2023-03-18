from commands_core import slash_command, client

def validate_guid(guid: str, base: int) -> int: 
    binary_guid = []

    binary_char_dyslexic = { 
        "0": b"00000",
        "1": b"00001",
        "2": b"00010",
        "3": b"00011",
        "4": b"00100",
        "5": b"00101",
        "6": b"00110",
        "7": b"00111",
        "8": b"01000",
        "9": b"01001",

        "a": b"01010",
        "b": b"01011",
        "c": b"01100",
        "d": b"01101",
        "e": b"01110",
        "f": b"01111",
        "g": b"10000",
        "h": b"10001",
        "j": b"10010",
        "k": b"10011",
        "m": b"10100",
        "n": b"10101",
        "p": b"10110",
        "q": b"10111",
        "r": b"11000",

        "s": b"11001",
        "t": b"11010",
        "u": b"11011",
        "v": b"11100",
        "w": b"11101",
        "x": b"11110",
        "y": b"11111",
    }

    if base == 32: 
        for i in len(guid): 
            binary_guid[i] = binary_char_dyslexic[guid[i]]
            pass
    elif base == 16:
        pass
    elif base == 10:
        pass
    elif base == 64:
        pass
    else: 
        client.print("GUID is not a valid base. Please use a base 16, 32, 64, or 10 GUID, either colon or hyphen seperated.")

@slash_command
def ban(args): # /ban {user} [duration] [duration]
    client.print("banned :D")

ban("/ban username 3d6h meow")