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
        
    def __add__(self, other):
        sum = []
        if isinstance(other, rgb):
            for x, y in zip(self.rgb, other.rgb):
                sum.append(x + y)
        elif type(other) == int:
            for x in self.rgb:
                sum.append(x + other)
        else:
            raise TypeError("other argument must be of type rgb")
            return None
        red, green, blue = sum
        return rgb(red, green, blue)
    
    def __sub__(self, other):
        sum = []
        if isinstance(other, rgb):
            for x, y in zip(self.rgb, other.rgb):
                sum.append(x - y)
        elif type(other) == int:
            for x in self.rgb:
                sum.append(x - other)
        else:
            raise TypeError("other argument must be of type rgb")
            return None
        red, green, blue = sum
        return rgb(red, green, blue)
            
    def to_hex(self):
        return '#%02x%02x%02x' % self.rgb
        
    def to_rgba(self, alpha=1):
        red, green, blue = self.rgb
        return rgba(red, green, blue, alpha)
        
class rgba:
    """
    Return an rgba representation of rgba in string
    """
    def __str__(self):
        return "rgba" + str(self.rgba)
        
    def __init__(self,  red, green, blue, alpha):
        self.rgba = (red, green, blue, alpha)
        
    def to_rgb(self):
        red, green, blue, alpha = self.rgba
        return rgb(red, green, blue)
        
    def __add__(self, other):
        sum = []
        if isinstance(other, rgba):
            for x, y in zip(self.rgba, other.rgba):
                sum.append(x + y)
        elif isinstance(other, als):
            for x in self.rgba[:-1]:
                sum.append(x + other.als[0])
            sum.append(self.rgba[-1] + other.als[1])
        else:
            raise TypeError("other argument must be of type rgba")
            return None
        red, green, blue, alpha = sum
        return rgba(red, green, blue, alpha)
    
    def __sub__(self, other):
        sum = []
        if isinstance(other, rgba):
            for x, y in zip(self.rgba, other.rgba):
                sum.append(x - y)
            red, green, blue = sum
            return rgba(red, green, blue)
        elif isinstance(other, als):
            for x in self.rgba[:-1]:
                sum.append(x - other.als[0])
            sum.append(self.rgba[-1] - other[1])
        else:
            raise TypeError("other argument must be of type rgba")
            return None
        red, green, blue, alpha = sum
        return rgba(red, green, blue, alpha)
        
class hsl:
    """
    Return an hsl representation of hsl in string
    """
    def __str__(self):
        return "hsl" + str(self.hsl)
        
    def __init__(self, hue, saturation, lightness):
        self.hsl = (hue, saturation, lightness)
        
    def to_hsla(self, alpha=1):
        hue, saturation, lightness = self.hsl
        return hsla(hue, saturation, lightness, alpha)
        
    def __add__(self, other):
        sum = []
        if isinstance(other, hsl):
            for x, y in zip(self.hsl, other.hsl):
                sum.append(x + y)
        elif type(other) == int:
            for x in self.hsl:
                sum.append(x + other)
        else:
            raise TypeError("other argument must be of type hsl")
            return None
        hue, saturation, lightness = sum
        return hsl(hue, saturation, lightness)
    
    def __sub__(self, other):
        sum = []
        if isinstance(other, hsl):
            for x, y in zip(self.hsl, other.hsl):
                sum.append(x - y)
        elif type(other) == int:
            for x in self.hsl:
                sum.append(x - other)
        else:
            raise TypeError("other argument must be of type hsl")
            return None
        hue, saturation, lightness = sum
        return hsl(hue, saturation, lightness)
        
class hsla:
    """
    Return an hsla representation of hsla in string
    """
    def __str__(self):
        return "hsla" + str(self.hsla)
        
    def __init__(self, hue, saturation, lightness, alpha):
        self.hsla = (hue, saturation, lightness, alpha)
        
    def to_hsl(self, alpha=1):
        hue, saturation, lightness, alpha = self.hsl
        return hsl(hue, saturation, lightness)
        
    def __add__(self, other):
        sum = []
        if isinstance(other, hsla):
            for x, y in zip(self.hsla, other.hsla):
                sum.append(x + y)
        elif isinstance(other, als):
            for x in self.hsla[:-1]:
                sum.append(x + other.als[0])
            sum.append(self.hsla[-1] + other.als[1])
        else:
            raise TypeError("other argument must be of type hsla")
            return None
        hue, saturation, lightness, alpha = sum
        return hsla(hue, saturation, lightness, alpha)
    
    def __sub__(self, other):
        sum = []
        if isinstance(other, hsla):
            for x, y in zip(self.hsla, other.hsla):
                sum.append(x - y)
        elif isinstance(other, als):
            for x in self.hsla[:-1]:
                sum.append(x - other.als[0])
            sum.append(self.hsla[-1] - other.als[1])
        else:
            raise TypeError("other argument must be of type hsla")
            return None
        hue, saturation, lightness, alpha = sum
        return hsla(hue, saturation, lightness, alpha)
        
class als:
    """
    A specal class mainly for alpha channel
    color arithmetic. The first elements are added 
    to all above representations and the secound
    is added to alpha channel
    """
    def __init__(self, num, alpha):
        self.als = (num, alpha)


print(rgba(20, 20, 20, 0.5) + als(20, 0.5))
