from formattoml.arrays import format_toml_arrays
from utils.path import get_workspace_root_path


def main():
    root = get_workspace_root_path()
    print(f"root: {root}")
    for toml_file in root.rglob("*.toml"):
        if toml_file.is_file():
            print(f"file path: {toml_file}")
            format_toml_arrays(toml_file)


if __name__ == "__main__":
    main()
