#!/usr/bin/env python
import importlib.util as il
import sys
import time
import os
from .features import Extras, check_pseudo_selector
from . import copy

class Main(Extras):
    content = ""
    watch_mode = False

    def get_dict_css(self):
        """
        Get the given filename and return
        the 'root' dict
        """
        spec = il.spec_from_file_location("mod", self.readfile)
        mod = il.module_from_spec(spec)
        spec.loader.exec_module(mod)
        return mod.root

    def get_properties(self, s, base):
        """
        Get properties and values from given dict
        """
        base_cp = base.copy()
        string = ""
        nests = []

        for i in s:
            base = base[i]
            string = check_pseudo_selector(i,  string)

        string += "{\n"
        for p, v in zip(base.keys(), base.values()):
            if not type(v) == dict:
                if isinstance(v, copy):
                    v = v.resolve(s, base_cp)
                string += f"    {p}: {v};\n"
            else:
                path = s.copy()
                path.append(p)
                nests.append(path)
        string += "}\n"
        return string, nests

    def writer(self, content):
        """
        Write 'contents' to file called 'writefile'
        """
        with open(self.writefile, "w") as f:
            f.write(content)

    def get_args(self, args):
        """ Parses the CLI args for options """
        if len(args) == 0:
            args = sys.argv[1:]
            if len(args) < 2:
                print("Reading file and writing file required")
                print("Syntax: pbss [-w] rf wf")
                sys.exit(1)
            if args[0] != "-w":
                self.readfile = os.path.expanduser(args[0])
                self.writefile = os.path.expanduser(args[1])
            else:
                self.watch_mode = True
                self.readfile = os.path.expanduser(args[1])
                self.writefile = os.path.expanduser(args[2])

    def parse_selectors(self, base):
        """
        Get the selectors and pass them
        to get_properties
        """
        for i in base.keys():
            block, nests = self.get_properties([i], base)
            self.content += block
            self.check_nests(nests, base)

    def execute(self):
        """
        Execute the programs step by step
        """
        data = self.get_dict_css()
        self.parse_selectors(data)
        self.writer(self.content)
        print(f"Compiled {self.readfile} and wrote to {self.writefile}")
        self.content = ""

    def __init__(self, *args):
        """
        Get the arguments and check for watch mode
        """
        if len(args) == 0:
            self.get_args(args)
        else:
            self.readfile = os.path.expanduser(args[0])
            self.writefile = os.path.expanduser(args[1])
        self.execute()
        if self.watch_mode:
            last_mod = os.stat(self.readfile).st_mtime
            while True:
                try:
                    c_time = os.stat(self.readfile).st_mtime
                    if last_mod == c_time:
                        time.sleep(1)
                    else:
                        self.execute()
                        last_mod = c_time
                except Exception as e:
                    print(e)
                    last_mod = c_time
