class Extras:
    def check_nests(self, nests, base):
        for ns in nests:
            block, nests = self.get_properties(ns, base)
            self.content += block

            if len(nests) > 0:
                self.check_nests(nests, base)
