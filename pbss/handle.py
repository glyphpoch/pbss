"""
Pbss's main file, handkes all of its execution
such as reading,and writing
"""

import importlib.util as il
import sys
from .file import File
from .parser import Parser

class Main:
    """
    Main class, all functionality are included inside it
    along with extra features
    """

    VERSION = "pbss-1.1  Python " + sys.version
    watch_mode = False
    quiet = False

    def get_dict_css(self):
        """
        Get the given filename and return
        the 'root' dict
        """
        spec = il.spec_from_file_location("mod", readfile)
        mod = il.module_from_spec(spec)
        spec.loader.exec_module(mod)
        return mod.root

    def get_args(self, args):
        """ Parses the CLI args for options """

        if "-v" in args:
            print(self.VERSION)
            sys.exit()
        if len(args) < 2:
            print("Reading file and writing file required")
            print("Syntax: pbss [-wq] rf wf")
            sys.exit(1)

        if "-w" in args:
            self.watch_mode = True
        if "-q" in args:
            self.quiet = True
        if "-wq" in args:
            self.quiet = True
            self.watch_mode = True

        self.readfile = File(args[-2], "r")
        self.writefile = File(args[-1], "w")

    def recompile(self):
        """
        Execute the programs step by step
        """
        data = self.readfile.get_module().root
        content = Parser(data).get_content()
        self.writefile.writer(content)

        if not self.quiet:
            print(f"Compiled {self.readfile} and wrote to {self.writefile}")

    def execute(self, *args):
        """
        Run to start the program and handles watch
        mode
        """
        if len(args) == 0:
            args = sys.argv[1:]

        self.get_args(args)
        self.recompile()
        if self.watch_mode:
            self.readfile.watch_file(self.recompile, self.quiet)