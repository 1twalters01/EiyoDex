import ast
import re
from pathlib import Path


def reformat_array_line(indent: str, key: str, eq_spacing: str, items_block: str) -> str:
    print(f"eq spacing: ={eq_spacing}=")
    key = key.rstrip() + " "
    eq_spacing = eq_spacing.lstrip().rstrip() + " "
    try:
        parsed_array = ast.literal_eval(f"[{items_block}]")
    except (SyntaxError, ValueError):
        return f"{indent}{key}{eq_spacing}[{items_block}]"

    if not parsed_array:
        return f"{indent}{key}{eq_spacing}[]"

    formatted_items = "".join(
        f"{indent}    {repr(item)},\n" for item in parsed_array
    )
    return f"{indent}{key}{eq_spacing}[\n{formatted_items}{indent}]"


def toml_array_replacer(match):
    return reformat_array_line(*match.groups())


def format_toml_arrays(file_path: Path):
    content = file_path.read_text(encoding="utf-8")

    regex_array_pattern = re.compile(
            r"^(\s*)([\"']?[A-Za-z0-9_\- ]+[\"']?)(\s*=\s*)\[(.*?)\]", re.DOTALL | re.MULTILINE)

    new_content = regex_array_pattern.sub(toml_array_replacer, content)

    if new_content != content:
        file_path.write_text(new_content, encoding="utf-8")
        print(f"Formatted {file_path}")
