"""
Pbss's main file, handkes all of its execution
such as reading,and writing
"""

import importlib.util as il
import sys
from .file import File
from .parser import Parser
from .watch import watch_file

class Main:
    """
    Main class, all functionality are included inside it
    along with extra features
    """
    watch_mode = False

    def get_dict_css(self):
        """
        Get the given filename and return
        the 'root' dict
        """
        readfile = str(self.readfile)
        spec = il.spec_from_file_location("mod", readfile)
        mod = il.module_from_spec(spec)
        spec.loader.exec_module(mod)
        return mod.root

    def writer(self, content):
        """
        Write 'contents' to file called 'writefile'
        by opening it as op_file (opened file)
        """
        writefile = str(self.writefile)
        with open(writefile, "w") as op_file:
            op_file.write(content)

    def get_args(self, args):
        """ Parses the CLI args for options """
        if len(args) == 0:
            args = sys.argv[1:]
            if len(args) < 2:
                print("Reading file and writing file required")
                print("Syntax: pbss [-w] rf wf")
                sys.exit(1)
            if args[0] == "-w":
                self.watch_mode = "f"
                self.readfile = File(args[1], "r")
                self.writefile = File(args[2], "w")
            else:
                self.readfile = File(args[0], "r")
                self.writefile = File(args[1], "w")
        else:
            self.readfile = File(args[0], "r")
            self.writefile = File(args[1], "w")

    def recompile(self):
        """
        Execute the programs step by step
        """
        data = self.get_dict_css()
        content = Parser(data).get_content()
        self.writer(content)
        print(f"Compiled {self.readfile} and wrote to {self.writefile}")

    def execute(self, *args):
        """
        Run to start the program and handles watch
        mode
        """
        self.get_args(args)
        self.recompile()
        if self.watch_mode == "f":
            watch_file(self.readfile, self.recompile)
