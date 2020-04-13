"""
Pbss module. This should be imported for access to all
pbss functionality instaed of importing each module

from pbss import *
"""
import importlib.util as il
from .units import *
from .colors import *
from .file import File

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

def cal(exp):
    ops = ["+", "-", "*", "/"]
    operator = None
    unit = []
    split_exp = None

    # Find operator and break the expression in digits
    for op in ops:
        if op in exp:
            split_exp = exp.split(op)
            operator = op
    # Find unit
    for char in split_exp[0]:
        if char.isalpha():
            unit.append(char)
    unit = "".join(unit)

    for index, digit in enumerate(split_exp):
        split_exp[index] = split_exp[index][:-len(unit)]
    # Return result
    expression = operator.join(split_exp)
    return eval(expression)

def attach(base, *args, placement="e"):
    master = {}

    if placement == "b":
        for k, val in zip(base.keys(), base.values()):
            master[k] = val
        for fn in args:
            if not fn.endswith(".py"):
                fn += ".py"
            rdict = File(fn, "r").get_module().root
            for k, val in zip(rdict.keys(), rdict.values()):
                master[k] = val

    if placement == "e":
        for fn in args:
            if not fn.endswith(".py"):
                fn += ".py"
            rdict = File(fn, "r").get_module().root
            for k, val in zip(rdict.keys(), rdict.values()):
                master[k] = val
        for k, val in zip(base.keys(), base.values()):
            master[k] = val

    return master