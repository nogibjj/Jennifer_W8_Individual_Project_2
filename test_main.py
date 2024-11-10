import unittest
from unittest.mock import patch
from main import track_performance
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import DBquery, CRUD_Create, CRUD_Read, CRUD_Update, CRUD_Delete


class TestMain(unittest.TestCase):
    @patch("mylib.extract.extract")
    def test_extract(self, mock_extract):
        """Test extract function performance tracking."""
        mock_extract.return_value = "data/goose_rawdata.csv"
        elapsed_time, memory_used, result = track_performance("Extract", extract)
        self.assertIsInstance(elapsed_time, float)
        self.assertIsInstance(memory_used, float)
        self.assertEqual(result, "data/goose_rawdata.csv")

    @patch("mylib.transform_load.load")
    def test_load(self, mock_load):
        """Test load function performance tracking."""
        mock_load.return_value = "GooseDB.db"
        elapsed_time, memory_used, result = track_performance("Load", load)
        self.assertIsInstance(elapsed_time, float)
        self.assertIsInstance(memory_used, float)
        self.assertEqual(result, "GooseDB.db")

    @patch("mylib.query.DBquery")
    def test_db_query(self, mock_dbquery):
        """Test DBquery function performance tracking."""
        mock_dbquery.return_value = "Query Successfully"
        elapsed_time, memory_used, result = track_performance("DB Query", DBquery)
        self.assertIsInstance(elapsed_time, float)
        self.assertIsInstance(memory_used, float)
        self.assertEqual(result, "Query Successfully")

    @patch("mylib.query.CRUD_Create")
    def test_crud_create(self, mock_create):
        """Test CRUD_Create function performance tracking."""
        mock_create.return_value = "Create Successfully"
        elapsed_time, memory_used, result = track_performance(
            "CRUD Create", CRUD_Create
        )
        self.assertIsInstance(elapsed_time, float)
        self.assertIsInstance(memory_used, float)
        self.assertEqual(result, "Create Successfully")

    @patch("mylib.query.CRUD_Read")
    def test_crud_read(self, mock_read):
        """Test CRUD_Read function performance tracking."""
        mock_read.return_value = "Read Successfully"
        elapsed_time, memory_used, result = track_performance("CRUD Read", CRUD_Read)
        self.assertIsInstance(elapsed_time, float)
        self.assertIsInstance(memory_used, float)
        self.assertEqual(result, "Read Successfully")

    @patch("mylib.query.CRUD_Update")
    def test_crud_update(self, mock_update):
        """Test CRUD_Update function performance tracking."""
        mock_update.return_value = "Update Successfully"
        elapsed_time, memory_used, result = track_performance(
            "CRUD Update", CRUD_Update
        )
        self.assertIsInstance(elapsed_time, float)
        self.assertIsInstance(memory_used, float)
        self.assertEqual(result, "Update Successfully")

    @patch("mylib.query.CRUD_Delete")
    def test_crud_delete(self, mock_delete):
        """Test CRUD_Delete function performance tracking."""
        mock_delete.return_value = "Delete Successfully"
        elapsed_time, memory_used, result = track_performance(
            "CRUD Delete", CRUD_Delete
        )
        self.assertIsInstance(elapsed_time, float)
        self.assertIsInstance(memory_used, float)
        self.assertEqual(result, "Delete Successfully")


if __name__ == "__main__":
    unittest.main()
