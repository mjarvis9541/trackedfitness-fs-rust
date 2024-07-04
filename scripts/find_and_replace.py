#!/usr/bin/env python3

import os
import re


def replace_pattern_in_files(directory):
    # Define the regex pattern to find and replace
    # This regex specifically captures the variable name used in the lambda function
    regex_pattern = re.compile(
        r"let (\w+) = move \|\| query\.with\(\|q\| q\.get\(\"(\w+)\"\)\.cloned\(\)\.unwrap_or_default\(\)\);"
    )
    replacement_template = 'let {0} = move || extract_param_str(&query, "{1}");'

    # Walk through all the files in the given directory
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(".rs"):  
                file_path = os.path.join(root, file)
                # Read the file content
                with open(file_path, "r", encoding="utf-8") as file:
                    file_content = file.read()

                # Find all matches and replace
                new_content = regex_pattern.sub(
                    lambda m: replacement_template.format(m.group(1), m.group(2)),
                    file_content,
                )
                if new_content != file_content:  # Check if there were any changes
                    # Write the new content back to the file
                    with open(file_path, "w", encoding="utf-8") as file:
                        file.write(new_content)
                    print(f"Replaced occurrences in {file_path}")


if __name__ == "__main__":
    project_directory = "/Users/michael/Development/trackedfitness-fs-rust/src"
    replace_pattern_in_files(project_directory)
