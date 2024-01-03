import json
import sys


CARGO_TOML = "src-tauri/Cargo.toml"
TAURI_CONF_JSON = "src-tauri/tauri.conf.json"






def get_cargo_version() -> tuple[int, int, int]:
    with open(CARGO_TOML, "r") as f:
        file = f.read()
        lines = file.split("\n")
        try:
            [line] = [line for line in lines if line.startswith("version = ")]
        except ValueError:
            raise ValueError("No version found in Cargo.toml")
        [_prefix, version_str, _blank] = line.split('"')
        try:
            [a, b, c] = map(int, version_str.split("."))
        except ValueError:
            raise ValueError(f"Invalid version string '{version_str}'")
        version = (a, b, c)
        return version
    
def set_cargo_version(version: tuple[int, int, int]):
    with open(CARGO_TOML, "r") as f:
        file = f.read()
        lines = file.split("\n")
        try:
            [line] = [line for line in lines if line.startswith("version = ")]
        except ValueError:
            raise ValueError("No version found in Cargo.toml")
        version_string = ".".join(map(str, version))
        new_line = f'version = "{version_string}"'
        file = file.replace(line, new_line)
    with open(CARGO_TOML, "w") as f:
        f.write(file)
    

def get_tauri_version() -> tuple[int, int, int]:
    with open(TAURI_CONF_JSON, "r") as f:
        tauri_conf = json.load(f)
    version_str = str(tauri_conf["package"]["version"])
    try:
        [a, b, c] = map(int, version_str.split("."))
    except ValueError:
        raise ValueError(f"Invalid version string '{version_str}'")
    version = (a, b, c)
    return version
    

def set_tauri_version(version: tuple[int, int, int]):
    with open(TAURI_CONF_JSON, "r") as f:
        tauri_conf = json.load(f)
    version_string = ".".join(map(str, version))
    tauri_conf["package"]["version"] = version_string
    with open(TAURI_CONF_JSON, "w") as f:
        json.dump(tauri_conf, f, indent=2)

def get_version() -> tuple[int, int, int]:
    cargo_version = get_cargo_version()
    tauri_version = get_tauri_version()
    version = max(cargo_version, tauri_version)
    set_cargo_version(version)
    set_tauri_version(version)
    return version


def set_version(version: tuple[int, int, int]):
    set_cargo_version(version)
    set_tauri_version(version)

def main():

    # if no arguments
    if len(sys.argv) == 1:
        version = get_version()
        print('.'.join(map(str, version)))

    # if one argument
    elif len(sys.argv) == 2:
        [a,b,c] = get_version()
        if sys.argv[1] == "+++":
            a += 1
            b = 0
            c = 0
        elif sys.argv[1] == "++":
            b += 1
            c = 0
        elif sys.argv[1] == "+":
            c += 1
        else:
            raise ValueError(f"Invalid argument '{sys.argv[1]}', must be one of '+++', '++', '+'")
        version = (a,b,c)
        set_version(version)
        print('.'.join(map(str, version)))

    else:
        raise ValueError("Too many arguments")

if __name__ == "__main__":
    main()