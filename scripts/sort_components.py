#!/usr/bin/env python3

import re
import sys
import os


def sort_rust_components(filename):
    try:
        with open(filename, "r") as file:
            content = file.read()
    except FileNotFoundError:
        print(f"Error: The file {filename} does not exist.")
        return

    # Regex to find each component block
    pattern = r"(\#\[(component)\][\s\S]*?}\s*})"

    # Find all component blocks in the file
    components = re.findall(pattern, content)

    # Extract the component blocks from the matches and strip excess whitespace
    component_blocks = [match[0].strip() for match in components]

    # Function to extract the component name
    def extract_name(component_block):
        name_match = re.search(r"pub fn (\w+)", component_block)
        if name_match:
            return name_match.group(1)
        return ""

    # Sort the components by name
    sorted_components = sorted(component_blocks, key=extract_name)

    # Combine the sorted components back into a single string
    sorted_content = "\n\n".join(sorted_components)

    # Prepare the output filename
    directory = os.path.dirname(filename)
    base_name = os.path.basename(filename)
    sorted_filename = os.path.join(directory, "sorted_" + base_name)

    # Create the directory if it doesn't exist
    if directory and not os.path.exists(directory):
        os.makedirs(directory)

    # Write the sorted contents back to the new file
    try:
        with open(sorted_filename, "w") as file:
            file.write(sorted_content)
        print(f"Components sorted successfully. Output file: {sorted_filename}")
    except IOError as e:
        print(f"Error writing to file: {e}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python sort_components.py <filename>")
    else:
        filename = sys.argv[1]
        sort_rust_components(filename)
