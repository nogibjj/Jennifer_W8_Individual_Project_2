"""
Extract a dataset from a URL to a file path

food dataset
"""

import os
import requests


def extract(
    url=(
        "https://github.com/fivethirtyeight/data/raw/refs/heads/master/"
        "goose/goose_rawdata.csv"
    ),
    file_path="data/goose_rawdata.csv",
):
    """Extract a URL to a file path"""
    os.makedirs(os.path.dirname(file_path), exist_ok=True)

    # Fetch the content from the URL
    response = requests.get(url)

    # Check for valid response status
    if response.status_code == 200:
        # Save the content to the specified file path
        with open(file_path, "wb") as f:
            f.write(response.content)
        print(f"File successfully downloaded to {file_path}")
    else:
        print(f"Failed to retrieve the file. HTTP Status Code: {response.status_code}")

    return file_path
