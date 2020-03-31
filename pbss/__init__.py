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
    root_dict = mod.root

    for k, val in zip(base.keys(), base.values()):
        root_dict[k] = val
    return root_dict

def rept(string, num):
    """ Returns a multiplication of `string` with `num` """

    line = (string + " ") * (num - 1) 
    line += string
    return line