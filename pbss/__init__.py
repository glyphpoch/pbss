class copy:
    def __init__(self, cp_sel,  prefix=None,  suffix=None):
        self.cp_sel = cp_sel
        self.prefix = prefix
        self.suffix = suffix
        
    def resolve(self, s, base):
        if self.cp_sel.startswith("/"):
            for i in s[:-1]:
                base = base[i]
            return str(base[self.cp_sel[1:]])
        if not " " in self.cp_sel:
            for i in s:
                base = base[i]
            return str(base[self.cp_sel])
        else:
            cp_sel = self.cp_sel.split(" ")
            for i in cp_sel[:-1]:
                base = base[i]
            return str(base[cp_sel[-1]])
