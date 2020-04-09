"""
Pbss module. This should be imported for access to all
pbss functionality instaed of importing each module

from pbss import *
"""
import importlib.util as il
from .units import *
from .colors import *

def rept(num, *args, delimiter=" "):
    """
    Returns a repeatative multiplicaion of a string
    """
    # First join the tuple with spaces and add one
    # extra space at end. Then multiply it num -1
    # so that there are no whitespace left
    # latsly join the tuple one more time with space
    # and add it to string

    args = list(args)

    for i in range(len(args)):
        args[i] = str(args[i])

    string = (delimiter.join(args) + delimiter) * (num - 1)
    string += delimiter.join(args)
    
    return string

def attach(base, *args, placement="e"):
    master = {}

    if placement == "b":
        for k, val in zip(base.keys(), base.values()):
            master[k] = val
        for fn in args:
            if not fn.endswith(".py"):
                fn += ".py"

            spec = il.spec_from_file_location("mod", fn)
            mod = il.module_from_spec(spec)
            spec.loader.exec_module(mod)
            rdict = mod.root

            for k, val in zip(rdict.keys(), rdict.values()):
                master[k] = val

    if placement == "e":
        for fn in args:
            if not fn.endswith(".py"):
                fn += ".py"

            spec = il.spec_from_file_location("mod", fn)
            mod = il.module_from_spec(spec)
            spec.loader.exec_module(mod)
            rdict = mod.root

            for k, val in zip(rdict.keys(), rdict.values()):
                master[k] = val
        for k, val in zip(base.keys(), base.values()):
            master[k] = val

    return master