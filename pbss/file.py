"""
Handles getting file path and additional features
such as modified time and filename in string form
"""
import os
import sys

class File:
    """ The main file class """
    def __init__(self, fpath, fmod):
        self.fpath = fpath
        if fmod == "r":
            self.file = os.path.expanduser(fpath)
            if not os.path.isfile(self.file):
                print(f"'{fpath}': No such file")
                sys.exit(1)
        if fmod == "w":
            self.file = os.path.expanduser(fpath)

    def get_mod_time(self):
        """ Returns the modified time of the file """
        self.mod_time = os.stat(self.file).st_time
        return self.mod_time

    def __str__(self):
        return self.fpath
