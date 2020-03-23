"""
Extra features for pbss like nesting and checking
for pseudo selectors
"""

class Extras:
    """
    Check if nests has any elements and run
    again if so
    """
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
            
