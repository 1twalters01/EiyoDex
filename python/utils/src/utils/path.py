from pathlib import Path


def get_workspace_root_path():
    return Path(__file__).parents[4]
