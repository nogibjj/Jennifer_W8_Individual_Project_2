"""
Transforms and Loads data into the local SQLite3 database

"""

import sqlite3
import csv
import os


# load the csv file and insert into a new sqlite3 database
def load(dataset="data/goose_rawdata.csv"):
    """ "Transforms and Loads data into the local SQLite3 database"""

    # prints the full working directory and path
    print(os.getcwd())
    dataset_path = os.path.abspath(dataset)
    if not os.path.exists(dataset_path):
        raise FileNotFoundError(f"{dataset_path} does not exist")

    payload = csv.reader(open(dataset, newline=""), delimiter=",")
    conn = sqlite3.connect("GooseDB.db")
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS GooseDB")
    c.execute(
        "CREATE TABLE GooseDB (name,year,team,league,goose_eggs,broken_eggs,mehs,"
        "league_average_gpct,ppf,replacement_gpct,gwar,key_retro)"
    )
    # insert
    c.executemany(
        "INSERT INTO GooseDB VALUES (?,?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", payload
    )
    conn.commit()
    conn.close()
    return "GooseDB.db"
