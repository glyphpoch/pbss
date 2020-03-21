import os
import sys

class File:
    def __init__(self, fp, ft):
        self.fp = fp
        if ft == "r":
            self.file = os.path.expanduser(fp)
            if not os.path.isfile(self.file):
                print(f"'{fp}': No such file")
                sys.exit(1)
        if ft == "w":
            self.file = os.path.expanduser(fp)
            
    def get_mod_time(self):
        self.mod_time = os.stat(self.file).st_time
        return self.mod_time
        
    def __str__(self):
        return self.fp
