"""
Pbss module. This should be imported for access to all
pbss functionality instaed of importing each module

from pbss import *
"""
import importlib.util as il
from .units import *
from .colors import *

def add(base, readfile):
    """
    Takes base as the root dict and reads readfile
    to get the root of that file. The readfile can
    have the .py extension but can be ommited
    """
    if not readfile.endswith(".py"):
        readfile += ".py"
    spec = il.spec_from_file_location("mod", readfile)
    mod = il.module_from_spec(spec)
    spec.loader.exec_module(mod)
    d = mod.root

    for k, v in zip(base.keys(), base.values()):
        d[k] = v
    return d

class copy:
    """
    Copy class. Copy cp_sel to where asked.
    Resolve requires s as a list to the path
    where cp_sel can be found whereas base is the
    root dicts
    """
    def __init__(self, cp_sel, prefix=None, suffix=None):
        self.cp_sel = cp_sel
        self.prefix = prefix
        self.suffix = suffix

    def resolve(self, s, base):
        """
        Resolve the path and get the value of the property
        """
        string = ""
        if self.cp_sel.startswith("/"):
            for i in s[:-1]:
                base = base[i]
            string = str(base[self.cp_sel[1:]])
        if not " " in self.cp_sel:
            for i in s:
                base = base[i]
            string = str(base[self.cp_sel])
        else:
            cp_sel = self.cp_sel.split(" ")
            for i in cp_sel[:-1]:
                base = base[i]
            string = str(base[cp_sel[-1]])
        return string
