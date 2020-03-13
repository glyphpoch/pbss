class rgb:
    """
    Return an rgb representation of rgb in string
    """
    def __str__(self):
        if self.hex is True:
            return self.to_hex()
        else:
            return "rgb" + str(self.rgb)
        
    def __init__(self,  red, green, blue, hex=False):
        self.rgb = (red, green, blue)
        self.hex = hex
        
    def to_hex(self):
        return '#%02x%02x%02x' % self.rgb
        
    def to_rgba(self, alpha=1):
        red, green, blue = self.rgb
        return rgba(red, green, blue, alpha)
        
class rgba:
    """
    Return an rgba representation of rgb in string
    """
    def __str__(self):
        return "rgba" + str(self.rgba)
        
    def __init__(self,  red, green, blue, alpha):
        self.rgba = (red, green, blue, alpha)
        
    def to_rgb(self):
        red, green, blue, alpha = self.rgba
        return rgb(red, green, blue)
        
class hsl:
    """
    Return an hsl representation of rgb in string
    """
    def __str__(self):
        return "hsl" + str(self.hsl)
        
    def __init__(self, hue, saturation, lightness):
        self.hsl = (hue, saturation, lightness)
        
    def to_hsla(self, alpha=1):
        hue, saturation, lightness = self.hsl
        return hsla(hue, saturation, lightness, alpha)
        
class hsla:
    """
    Return an hsla representation of rgb in string
    """
    def __str__(self):
        return "hsla" + str(self.hsla)
        
    def __init__(self, hue, saturation, lightness, alpha):
        self.hsla = (hue, saturation, lightness, alpha)
        
    def to_hsl(self, alpha=1):
        hue, saturation, lightness, alpha = self.hsl
        return hsl(hue, saturation, lightness)
