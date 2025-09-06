import sys
import json
import os

def replace_user(obj):
    if isinstance(obj, dict):
        new_obj = {}
        for k, v in obj.items():
            if k == "User" or k == "Wrapper":
                return v
            else:
                new_obj[k] = replace_user(v)
        return new_obj
    elif isinstance(obj, list):
        return [replace_user(item) for item in obj]
    else:
        return obj

def process_file(filepath, overwrite=False):
    with open(filepath, "r", encoding="utf-8") as f:
        data = json.load(f)
    updated = replace_user(data)
    if overwrite:
        with open(filepath, "w", encoding="utf-8") as f:
            json.dump(updated, f, indent=4)
    else:
        print(json.dumps(updated, indent=4))

def process_directory(directory):
    for root, _, files in os.walk(directory):
        for name in files:
            if name.lower().endswith(".json"):
                filepath = os.path.join(root, name)
                try:
                    process_file(filepath, overwrite=True)
                except Exception as e:
                    print(f"Error processing {filepath}: {e}")

def main():
    if len(sys.argv) != 2:
        print("Usage: python script.py <filename or directory>")
        sys.exit(1)
    path = sys.argv[1]
    if os.path.isdir(path):
        process_directory(path)
    elif os.path.isfile(path):
        process_file(path)
    else:
        print("Provided path is not a file or directory.")
        sys.exit(1)

if __name__ == "__main__":
    main()