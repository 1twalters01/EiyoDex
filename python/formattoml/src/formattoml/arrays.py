from pathlib import Path
import re
import ast


def reformat_array_line(indent: str, key: str, items_block: str) -> str:
    try:
        parsed_array = ast.literal_eval(f"[{items_block}]")
    except (SyntaxError, ValueError):
        return f"{indent}{key} = [\n{items_block}]"

    if not parsed_array:
        return f"{indent}{key} = []"

    # Reconstruct array with 4-space indentation and trailing commas
    formatted_items = "".join(
        f"{indent}    {repr(item)},\n" for item in parsed_array
    )
    return f"{indent}{key} = [\n{formatted_items}{indent}]"


def toml_array_replacer(match):
    return reformat_array_line(*match.groups())


def format_toml_arrays(file_path: Path):
    content = file_path.read_text(encoding="utf-8")

    regex_array_pattern = re.compile(
        r"^(\s*)(\w+)\s*=\s*\[(.*?)\]", re.DOTALL | re.MULTILINE
    )

    new_content = regex_array_pattern.sub(toml_array_replacer, content)

    if new_content != content:
        file_path.write_text(new_content, encoding="utf-8")
        print(f"Formatted {file_path}")
