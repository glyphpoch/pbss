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
