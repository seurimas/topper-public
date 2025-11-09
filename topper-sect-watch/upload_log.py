#!/usr/bin/env python3
"""
Upload a JSON log file to the Supabase edge function.

Usage:
    python upload_log.py <url> <jwt_token> <filename_or_directory> <api_key>
"""

import sys
import json
import os
from pathlib import Path
import requests


def upload_log(filename: str, url: str, jwt_token: str, api_key: str) -> bool:
    """
    Read JSON from file and send it to the Supabase edge function.
    
    Args:
        filename: Path to the JSON file to upload
        url: URL of the edge function
        jwt_token: JWT token for authorization
        api_key: API key for authentication
        
    Returns:
        True if upload was successful, False otherwise
    """
    # Read the JSON file
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            page_data = json.load(f)
    except FileNotFoundError:
        print(f"Error: File '{filename}' not found.", file=sys.stderr)
        return False
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in '{filename}': {e}", file=sys.stderr)
        return False
    
    # Prepare the request
    headers = {
        'Authorization': f'Bearer {jwt_token}',
        'Content-Type': 'application/json'
    }
    payload = {
        'apiKey': api_key,
        'page': page_data
    }
    
    # Send the request
    try:
        response = requests.post(url, headers=headers, json=payload)
        
        # Print response details
        print(f"Status Code: {response.status_code}")
        print(f"Response Headers: {dict(response.headers)}")
        print(f"Response Body: {response.text}")
        
        if response.ok:
            print(f"\n✓ Successfully uploaded '{filename}'")
            return True
        else:
            print(f"\n✗ Upload failed with status {response.status_code}", file=sys.stderr)
            return False
            
    except requests.RequestException as e:
        print(f"Error: Request failed: {e}", file=sys.stderr)
        return False


def process_path(path: str, url: str, jwt_token: str, api_key: str) -> None:
    """
    Process a file or directory path. If it's a directory, upload all JSON files.
    
    Args:
        path: Path to a file or directory
        url: URL of the edge function
        jwt_token: JWT token for authorization
        api_key: API key for authentication
    """
    path_obj = Path(path)
    
    if not path_obj.exists():
        print(f"Error: Path '{path}' does not exist.", file=sys.stderr)
        sys.exit(1)
    
    if path_obj.is_file():
        # Single file
        success = upload_log(str(path_obj), url, jwt_token, api_key)
        if not success:
            sys.exit(1)
    elif path_obj.is_dir():
        # Directory - process all JSON files
        json_files = sorted(path_obj.glob('*.json'))
        
        if not json_files:
            print(f"No JSON files found in directory '{path}'", file=sys.stderr)
            sys.exit(1)
        
        print(f"Found {len(json_files)} JSON file(s) in '{path}'")
        
        success_count = 0
        failed_count = 0
        
        for json_file in json_files:
            print(f"\n--- Processing {json_file.name} ---")
            if upload_log(str(json_file), url, jwt_token, api_key):
                success_count += 1
            else:
                failed_count += 1
        
        print(f"\n=== Summary ===")
        print(f"Successfully uploaded: {success_count}")
        print(f"Failed: {failed_count}")
        
        if failed_count > 0:
            sys.exit(1)
    else:
        print(f"Error: '{path}' is neither a file nor a directory.", file=sys.stderr)
        sys.exit(1)


def main():
    if len(sys.argv) != 5:
        print("Usage: python upload_log.py <url> <jwt_token> <filename_or_directory> <api_key>", file=sys.stderr)
        sys.exit(1)
    
    url = sys.argv[1]
    jwt_token = sys.argv[2]
    path = sys.argv[3]
    api_key = sys.argv[4]
    
    process_path(path, url, jwt_token, api_key)


if __name__ == '__main__':
    main()
