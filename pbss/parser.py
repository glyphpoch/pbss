"""
File containg the main parsing engine
"""

from .properties import Property

class Parser:
    content = ""
    @classmethod
    def check_pseudo_selector(cls, i, string):
        """
        Check if the given i is a pueudo
        sellector. Having : in front
        """
        if i.startswith(":"):
            string = string[:-1]
        string += i + " "
        return string

    def at_ops(self, sel, base):
        """
        Handle @ commands like @media, @keyframe
        """
        if sel.startswith("@"):
            string = "" + sel + " {\n"
            for k in base[sel]:
                block, nests = self.get_properties([k], base[sel])
                for line in block.split("\n")[:-1]:
                    string += "    " + line + "\n"
            string += "}\n"
            self.check_nests(nests, base[sel])
            self.content += string
            return True
        return False

    def check_nests(self, nests, base):
        """
        Check if length of nests > 0
        if so reexecute get_properties
        """
        for nest_sel in nests:
            block, nests = self.get_properties(nest_sel, base)
            self.content += block

            if len(nests) > 0:
                self.check_nests(nests, base)

    def get_properties(self, sel, base):
        """
        Get properties and values from given dict
        """
        string = ""
        nests = []

        for i in sel:
            base = base[i]
            string = self.check_pseudo_selector(i, string)

        string += "{\n"
        for prop, val in zip(base.keys(), base.values()):
            if not isinstance(val, dict):
                if Property().exist(prop):
                    string += f"    {prop}: {val};\n"
            else:
                path = sel.copy()
                path.append(prop)
                nests.append(path)
        string += "}\n"
        return string, nests
        
    def __init__(self, base):
        """
        Get the selectors and pass them
        to get_properties
        """
        for i in base.keys():
            at_sel = self.at_ops(i, base)
            if at_sel:
                continue
            block, nests = self.get_properties([i], base)
            self.content += block
            self.check_nests(nests, base)
        
    def get_content(self):
        return self.content
