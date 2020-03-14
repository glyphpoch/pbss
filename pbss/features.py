class Extras:
    def check_nests(self, nests, base):
        for ns in nests:
            block, nests = self.get_properties(ns, base)
            self.content += block

            if len(nests) > 0:
                self.check_nests(nests, base)
                
    def check_pseudo_selector(self,  i,  string):
        """
        Check if the given i is a pueudo
        sellector. Having : in front
        """
        if i.startswith(":"):
            string = string[:-1]
        string += i + " "
        return string
