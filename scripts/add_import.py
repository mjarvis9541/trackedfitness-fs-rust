#!/usr/bin/env python3

import os
import re

project_path = "/Users/michael/Development/trackedfitness-fs-rust/src"

# Regex pattern to search for
pattern = re.compile(
    r"let search = move \|\| query\.with\(\|q\| q\.get\(\"search\"\)\.cloned\(\)\.unwrap_or_default\(\)\);"
)

# Line to add
line_to_add = "use crate::util::param::{extract_param, extract_param_str, is_valid_page, is_valid_size};\n"

# Iterate over the files in the directory
for root, dirs, files in os.walk(project_path):
    for file in files:
        if file.endswith(".rs"): 
            file_path = os.path.join(root, file)
            with open(file_path, "r+") as f:
                contents = f.readlines()
                for i, line in enumerate(contents):
                    if pattern.search(line):
                        if (
                            line_to_add not in contents[0]
                        ):  # Check if the import line is already there
                            contents.insert(0, line_to_add)  # Insert at the top
                            f.seek(0)  # Go back to the start of the file
                            f.writelines(contents)  # Write the modified contents
                            break
