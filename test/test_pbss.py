import unittest
from pbss.handle import Main

class TestPbss(unittest.TestCase):
    def test_pbss(self):
        Main().execute("test/style.py",  "test/style.css")
        with open("test/actual.css") as r:
            result = r.read()
        with open("test/style.css") as s:
            prediction = s.read()
            
        self.assertEqual(result, prediction)
        
if __name__ == "__main__":
    unittest.main()
