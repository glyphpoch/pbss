#!/usr/bin/env python
import importlib.util as il
import sys
import time
import os
from .features import Extras

class Main(Extras):
    content = ""
    watch_mode = False

    def get_dict_css(self):
        """
        Get the given filename and return the 'root' dict
        """
        spec = il.spec_from_file_location("mod", self.readfile)
        mod = il.module_from_spec(spec)
        spec.loader.exec_module(mod)
        return mod.root
        
    def check_pseudo_selector(self,  i,  string):
        if i.startswith(":"):
            string = string[:-1]
        string += i + " "
        return string

    def get_properties(self, s, base):
        """
        Get properties and values from given dict
        """
        string = ""
        nests = []

        for i in s:
            base = base[i]
            string = self.check_pseudo_selector(i,  string)

        string += "{\n"
        for p, v in zip(base.keys(), base.values()):
            if not type(v) == dict:
                var = super().copies(p, base)
                if not var is None:
                    string += var
                    continue
                    
                string += f"    {p}: {v};\n"
            else:
                path = s.copy()
                path.append(p)
                nests.append(path)
        string += "}\n"
        return string, nests

    def writer(self, content):
        """
        Write 'contents' to file called 'wf'
        """
        with open(self.writefile, "w") as f:
            f.write(content)

    def get_args(self, args):
        """ Parses the CLI args for options """
        """pbss [--w] rf wf"""
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
        for i in base.keys():
            block, nests = self.get_properties([i], base)
            self.content += block
            self.check_nests(nests, base)

    def execute(self):
        data = self.get_dict_css()
        self.parse_selectors(data)
        self.writer(self.content)
        print(f"Compiled {self.readfile} and wrote to {self.writefile}")
        self.content = ""

    def __init__(self, *args):
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
                    a_time = os.stat(self.readfile).st_mtime
                    if last_mod == a_time:
                        time.sleep(1)
                    else:
                        self.execute()
                        last_mod = a_time
                except Exception as e:
                    print(e)
                    last_mod = a_time
