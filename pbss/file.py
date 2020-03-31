"""
Handles getting file path and additional features
such as modified time and filename in string form
"""
import sys, time, os

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
        self.mod_time = os.stat(self.file).st_mtime
        return self.mod_time

    def __str__(self):
        return self.fpath

    def watch_file(self, func):
        last_mod = self.get_mod_time()
        while True:
            try:
                c_time = self.get_mod_time()
                if last_mod == c_time:
                    time.sleep(1)
                else:
                    func()
                    last_mod = c_time
            except KeyboardInterrupt:
                sys.exit(0)
            except Exception as excep:
                print(excep)
                last_mod = c_time
