import tomli
import json


CARGO_TOML = "src-tauri/Cargo.toml"
TAURI_CONF_JSON = "src-tauri/tauri.conf.json"




def get_cargo_version():
    with open(CARGO_TOML, "rb") as f:
        cargo = tomli.load(f)
        version = cargo["package"]["version"]
        return version
    

def get_tauri_version():
    with open(TAURI_CONF_JSON, "rb") as f:
        tauri_conf = tomli.load(f)
        version = tauri_conf["package"]["version"]
        return version
    
def set_tauri_version(version: str):
    with open(TAURI_CONF_JSON, "r") as f:
        tauri_conf = json.load(f)
    tauri_conf["package"]["version"] = version
    with open(TAURI_CONF_JSON, "w") as f:
        json.dump(tauri_conf, f, indent=2)

# read version from tomli
with open("src-tauri/Cargo.toml", "rb") as f:
    cargo = tomli.load(f)
    version = cargo["package"]["version"]
    print(version)


def main():
    version = get_cargo_version()
    set_tauri_version(version)
    print(version)

if __name__ == "__main__":
    main()