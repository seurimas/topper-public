import sys
import json
import os

def replace_key(obj, key_to_replace):
    if isinstance(obj, dict):
        # If dict has only one key and it matches, replace with its value
        if len(obj) == 1 and key_to_replace in obj:
            return obj[key_to_replace]
        else:
            return {k: replace_key(v, key_to_replace) for k, v in obj.items()}
    elif isinstance(obj, list):
        return [replace_key(item, key_to_replace) for item in obj]
    else:
        return obj

def process_file(filepath, key_to_replace, overwrite=False):
    with open(filepath, "r", encoding="utf-8") as f:
        data = json.load(f)
    updated = replace_key(data, key_to_replace)
    if overwrite:
        with open(filepath, "w", encoding="utf-8") as f:
            json.dump(updated, f, indent=4)
    else:
        print(json.dumps(updated, indent=4))

def process_directory(directory, key_to_replace):
    for root, _, files in os.walk(directory):
        for name in files:
            if name.lower().endswith(".json"):
                filepath = os.path.join(root, name)
                process_file(filepath, key_to_replace, overwrite=True)

def main():
    if len(sys.argv) < 2 or len(sys.argv) > 3:
        print("Usage: python script.py <filename or directory> [key_to_replace]")
        sys.exit(1)
    path = sys.argv[1]
    key_to_replace = sys.argv[2] if len(sys.argv) == 3 else "User"
    if os.path.isdir(path):
        process_directory(path, key_to_replace)
    elif os.path.isfile(path):
        process_file(path, key_to_replace)
    else:
        print("Provided path is not a file or directory.")
        sys.exit(1)

if __name__ == "__main__":
    main()