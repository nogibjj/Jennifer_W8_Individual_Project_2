"""Query the database"""

import sqlite3


def DBquery():
    """Query the database for the top 10 rows of the GooseDB table"""
    conn = sqlite3.connect("GooseDB.db")
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM GooseDB LIMIT 10")
    print("Top 10 rows of the GooseDB table:")
    print(cursor.fetchall())
    conn.close()
    return "Query Successfully"


def CRUD_Create():
    conn = sqlite3.connect("GooseDB.db")
    cursor = conn.cursor()
    # create execution
    cursor.execute(
        "INSERT INTO GooseDB (name,year,team,league,goose_eggs,broken_eggs,mehs,"
        "league_average_gpct,ppf,replacement_gpct,gwar,key_retro)"
        "VALUES ('Jennifer Li', 2024, 'DKU', 'AL', 0, 0, 0, 0.0, 0.0, 0.0, 0.0, "
        "'jennifer101')"
    )
    conn.commit()
    conn.close()
    return "Create Successfully"


def CRUD_Read():
    conn = sqlite3.connect("GooseDB.db")
    cursor = conn.cursor()
    # read execution
    cursor.execute("SELECT * FROM GooseDB LIMIT 10")
    conn.close()
    return "Read Successfully"


def CRUD_Update():
    conn = sqlite3.connect("GooseDB.db")
    cursor = conn.cursor()
    # update execution
    cursor.execute("UPDATE GooseDB SET year = 2024 WHERE key_retro = 'luqud101'")
    conn.commit()
    conn.close()
    return "Update Successfully"


def CRUD_Delete():
    conn = sqlite3.connect("GooseDB.db")
    cursor = conn.cursor()
    # delete execution
    cursor.execute("DELETE FROM GooseDB WHERE key_retro = 'kircm101'")
    conn.commit()
    conn.close()
    return "Delete Successfully"


if __name__ == "__main__":
    DBquery()
    CRUD_Create()
    CRUD_Read()
    CRUD_Update()
    CRUD_Delete()
