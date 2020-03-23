"""
Pbss's main file, handkes all of its functionality
such as reading, parsing, formatting and writing
"""

#!/usr/bin/env python
import importlib.util as il
import sys
import time
from .file import File
from .features import Extras

class Main(Extras):
    """
    Main class, all functionality are included inside it
    along with extra features
    """
    content = ""
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

    def get_properties(self, sel, base):
        """
        Get properties and values from given dict
        """
        string = ""
        nests = []

        for i in sel:
            base = base[i]
            string = super().check_pseudo_selector(i, string)

        string += "{\n"
        for prop, val in zip(base.keys(), base.values()):
            if not isinstance(val, dict):
                string += f"    {prop}: {val};\n"
            else:
                path = sel.copy()
                path.append(prop)
                nests.append(path)
        string += "}\n"
        return string, nests

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
            if args[0] != "-w":
                self.readfile = File(args[0], "r")
                self.writefile = File(args[1], "w")
            else:
                self.watch_mode = True
                self.readfile = File(args[1], "r")
                self.writefile = File(args[2], "w")
        else:
            self.readfile = File(args[0], "r")
            self.writefile = File(args[1], "w")

    def parse_selectors(self, base):
        """
        Get the selectors and pass them
        to get_properties
        """
        for i in base.keys():
            at_sel = super().at_ops(i, base)
            if at_sel:
                continue
            block, nests = self.get_properties([i], base)
            self.content += block
            self.check_nests(nests, base)

    def recompile(self):
        """
        Execute the programs step by step
        """
        data = self.get_dict_css()
        self.parse_selectors(data)
        self.writer(self.content)
        print(f"Compiled {self.readfile} and wrote to {self.writefile}")
        self.content = ""

    def execute(self, *args):
        """
        Run to start the program and handles watch
        mode
        """
        self.get_args(args)
        self.recompile()
        if self.watch_mode:
            last_mod = self.readfile.get_mod_time()
            while True:
                try:
                    c_time = self.readfile.get_mod_time()
                    if last_mod == c_time:
                        time.sleep(1)
                    else:
                        self.recompile()
                        last_mod = c_time
                except KeyboardInterrupt:
                    sys.exit(0)
                except Exception as excep:
                    print(excep)
                    last_mod = c_time
