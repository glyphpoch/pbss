# CSS Arithmetic
Pbss can handle color arithmetic which means that two colors of the same class can be added together, In order to use these, first import pbss
``` python
from pbss import *
```
let's add 2 rgb colors in this example
``` python
root = {
    "div": {
        "background": rgb(20,20,20) + rgb(50,50,50),
    }
}
```
This generates the following CSS
``` CSS
div {
    background: rgb(70,70,70);
}
```
__It is to note that only addition and subtraction are supported
Also colors of same class can be added together like this__
* __rgb + rgb__
* __rgba + rgba__
* __hsl + hsl__
* __hsla + hsla__


* __rgb - rgb__
* __rgba - rgba__
* __hsl - hsl__
* __hsla - hsla__

If you want to add the one value to all the values of colors for rgb and hsl, you can add it like an integer for example, the above example can be written as
``` python
root = {
    "div": {
        "background": rgb(20,20,20) + 50
    }
}
```
This too generates the same CSS
``` CSS
div {
    background: rgb(70,70,70);
}
```

## Als Class: Alpha arithmetic
Als is a special class that takes two arguments, the first argument is added to all the colors of rgba or hsla and the secound argument is added to the alpha channel of the color, this is only made for alpha arithmetic since alpha ranges from 0.0 to 1.0. For example let's add 20 to all the colors and 0.2 to the alpha channel in this example
``` python
root = {
    "div": {
        "background": rgba(60,60,60,0.8) + als(20,0.2)
    }
}
```
This generates the following CSS
``` CSS
div {
    background: rgba(80,80,80,0.8);
}
```
__Note that this class is strictly implemented for alpha arithmetic__