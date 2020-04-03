"""
Implements all colors supported by CSS by giving a
class based representation of the colors. Supports
rgb, rgba, hsl and hsla
"""

class rgb:
    """
    Return an rgb representation of rgb in string
    """
    def __str__(self):
        if self.is_hex is True:
            return self.to_hex()
        return "rgb" + str(self.rgb)

    def __repr__(self):
        return "rgb" + str(self.rgb)

    def __init__(self, red, green, blue, is_hex=False):
        self.rgb = (red, green, blue)
        self.is_hex = is_hex

    def __add__(self, other):
        result = []
        if isinstance(other, rgb):
            for num1, num2 in zip(self.rgb, other.rgb):
                result.append(num1 + num2)
        elif isinstance(other, int):
            for num in self.rgb:
                result.append(num + other)
        else:
            raise TypeError("other argument must be of type rgb")

        red, green, blue = result
        return rgb(red, green, blue)

    def __sub__(self, other):
        result = []
        if isinstance(other, rgb):
            for num1, num2 in zip(self.rgb, other.rgb):
                result.append(num1 - num2)
        elif isinstance(other, int):
            for num in self.rgb:
                result.append(num - other)
        else:
            raise TypeError("other argument must be of type rgb")

        red, green, blue = result
        return rgb(red, green, blue)

    def to_hex(self):
        """
        Converts a given rgb value to hex value
        """
        return '#%02x%02x%02x' % self.rgb

    def to_rgba(self, alpha=1):
        """
        Returns a rgba object by adding alpha. Custom
        value can be passed along with the alpha argument
        """
        red, green, blue = self.rgb
        return rgba(red, green, blue, alpha)

class rgba:
    """
    Return an rgba representation of rgba in string
    """
    def __str__(self):
        return "rgba" + str(self.rgba)

    def __repr__(self):
        return "rgba" + str(self.rgba)

    def __init__(self, red, green, blue, alpha):
        self.rgba = (red, green, blue, alpha)

    def to_rgb(self):
        """
        Returns a rgb object by removing the alpha channel
        """
        red, green, blue, _ = self.rgba
        return rgb(red, green, blue)

    def __add__(self, other):
        result = []
        if isinstance(other, rgba):
            for num1, num2 in zip(self.rgba, other.rgba):
                result.append(num1 + num2)
        elif isinstance(other, als):
            return NotImplemented
        else:
            raise TypeError("other argument must be of type rgba")

        red, green, blue, alpha = result
        return rgba(red, green, blue, alpha)

    def __sub__(self, other):
        result = []
        if isinstance(other, rgba):
            for num1, num2 in zip(self.rgba, other.rgba):
                result.append(num1 - num2)
        elif isinstance(other, als):
            return NotImplemented
        else:
            raise TypeError("other argument must be of type rgba")

        red, green, blue, alpha = result
        return rgba(red, green, blue, alpha)

class hsl:
    """
    Return an hsl representation of hsl in string
    """
    def __str__(self):
        return "hsl" + str(self.hsl)

    def __repr__(self):
        return "hsl" + str(self.hsl)

    def __init__(self, hue, saturation, lightness):
        self.hsl = (hue, saturation, lightness)

    def to_hsla(self, alpha=1):
        """
        Returns a hsla object by adding alpha. Custom
        value can be passed along with the alpha argument
        """
        hue, saturation, lightness = self.hsl
        return hsla(hue, saturation, lightness, alpha)

    def __add__(self, other):
        result = []
        if isinstance(other, hsl):
            for num1, num2 in zip(self.hsl, other.hsl):
                result.append(num1 + num2)
        elif isinstance(other, int):
            for num in self.hsl:
                result.append(num + other)
        else:
            raise TypeError("other argument must be of type hsl")

        hue, saturation, lightness = result
        return hsl(hue, saturation, lightness)

    def __sub__(self, other):
        result = []
        if isinstance(other, hsl):
            for num1, num2 in zip(self.hsl, other.hsl):
                result.append(num1 - num2)
        elif isinstance(other, int):
            for num in self.hsl:
                result.append(num - other)
        else:
            raise TypeError("other argument must be of type hsl")

        hue, saturation, lightness = result
        return hsl(hue, saturation, lightness)

class hsla:
    """
    Return an hsla representation of hsla in string
    """
    def __str__(self):
        return "hsla" + str(self.hsla)

    def __repr__(self):
        return "hsla" + str(self.hsla)

    def __init__(self, hue, saturation, lightness, alpha):
        self.hsla = (hue, saturation, lightness, alpha)

    def to_hsl(self):
        """
        Returns a hsl object by removing the alpha channel
        """
        hue, saturation, lightness, _ = self.hsla
        return hsl(hue, saturation, lightness)

    def __add__(self, other):
        result = []
        if isinstance(other, hsla):
            for num1, num2 in zip(self.hsla, other.hsla):
                result.append(num1 + num2)
        elif isinstance(other, als):
            return NotImplemented
        else:
            raise TypeError("other argument must be of type hsla")

        hue, saturation, lightness, alpha = result
        return hsla(hue, saturation, lightness, alpha)

    def __sub__(self, other):
        result = []
        if isinstance(other, hsla):
            for num1, num2 in zip(self.hsla, other.hsla):
                result.append(num1 - num2)
        elif isinstance(other, als):
            return NotImplemented
        else:
            raise TypeError("other argument must be of type hsla")

        hue, saturation, lightness, alpha = result
        return hsla(hue, saturation, lightness, alpha)

class als:
    """
    A specal class mainly for alpha channe
    color arithmetic. The first elements are added
    to all above representations and the secound
    is added to alpha channel
    """
    def __init__(self, num, alpha):
        self.als = (num, alpha)

    def __radd__(self, other):
        result = []
        if isinstance(other, rgba):
            for num in other.rgba[:-1]:
                result.append(num + self.als[0])
            result.append(other.rgba[-1] + self.als[1])
            red, green, blue, alpha = result
            result = rgba(red, green, blue, alpha)

        elif isinstance(other, hsla):
            for num in other[:-1]:
                result.append(num + self.als[0])
            result.append(other[-1] + self.als[1])
            hue, saturation, lightness, alpha = result
            result = rgba(hue, saturation, lightness, alpha)

        return result

    def __rsub__(self, other):
        result = []
        if isinstance(other, rgba):
            for num in other.rgba[:-1]:
                result.append(num - self.als[0])
            result.append(other.rgba[-1] - self.als[1])
            red, green, blue, alpha = result
            result = rgba(red, green, blue, alpha)

        elif isinstance(other, hsla):
            for num in other[:-1]:
                result.append(num - self.als[0])
            result.append(other[-1] - self.als[1])
            hue, saturation, lightness, alpha = result
            result = rgba(hue, saturation, lightness, alpha)

        return result
