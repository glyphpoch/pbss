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

    def check_pseudo_selector(self, i, string):
        """
        Check if the given i is a pueudo
        sellector. Having : in front
        """
        if i.startswith(":"):
            string = string[:-1]
        string += i + " "
        return string
        
    def at_ops(self, s, base):
        """
        Handle @ commands like @media, @keyframe
        """
        if s.startswith("@"):
            string = "" + s + " {\n"
            for k in base[s]:
                block, nests = self.get_properties([k], base[s])
                for l in block.split("\n"):
                    string += "    " + l + "\n"
            string += "}\n"
            self.content += string
            self.check_nests(nests, base[s])
            return True
