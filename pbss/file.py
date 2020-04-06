"""
Handles getting file path and additional features
such as modified time and filename in string form
"""
import sys, time, os
import importlib.util as il

class FileTypeError(Exception):

    def __init__(self, msg):
        super().__init__(msg)

class File:
    """ The main file class """
    def __init__(self, fpath, fmod):
        self.fpath = fpath
        self.fmod = fmod
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

    def watch_file(self, func, quiet=False):
        if not self.fmod == "r":
            raise FileTypeError("watch_file called on a file that's not opened in reading mode")
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
                if not quiet:
                    print(excep)
                    last_mod = c_time

    def get_dict_css(self):
        """
        Get the given filename and return
        the 'root' dict
        """
        if not self.fmod == "r":
            raise FileTypeError("get_dict_css called on a file that's not opened in reading mode")

        spec = il.spec_from_file_location("mod", self.file)
        mod = il.module_from_spec(spec)
        spec.loader.exec_module(mod)
        return mod.root

    def writer(self, content):
        """
        Write 'contents' to file called 'writefile'
        by opening it as op_file (opened file)
        """
        if not self.fmod == "w":
            raise FileTypeError("writer called on a file that's not opened in writing mode")

        with open(self.file, "w") as op_file:
            op_file.write(content)