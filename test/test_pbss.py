import unittest
from pbss.handle import Main
from .style import root

class TestPbss(unittest.TestCase):

	m = Main()
	
	def test_get_args(self):
		self.m.get_args(("test/style.py", "test/style.css"))
	
	def test_get_dict_css(self):
		self.maxDiff = None
		adict = self.m.get_dict_css()

	def test_recompile(self):
		self.m.recompile()

if __name__ == "__main__":
	unittest.main()