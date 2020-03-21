# CSS Units
Pbss supports all CSS units by default like _em_, _rem_, _vh_, etc. But they are not available directly to your python file. To import all of them, you import pbss's _ _init_ _ file. To do so, use this syntax
``` python
from pbss import *
```
Though this syntax is discouraged by PEP-8 standards, but it is the only way of importing everything required for harnessing Pbss's full power without importing each function

## CSS Lengths

Let's declare the height and width of a div using this
```  python
from pbss import *

root = {
    "div": {
        "width": pct(50);
        "height": em(30);
    }
}
```
__Note that pct stands for % sign__  
This would generate the following code
``` CSS
div {
    width: 50%;
    height: 30em;
}
```
The following length units are supported. The arrows denote how they will be compiled to CSS units
* mm -> mm
* cm -> cm
* inc -> in (inch)
* px -> px
* pt -> pt
* pc -> pc
* em -> em
* ex -> ex
* ch -> ch
* rem -> rem
* vw -> vw
* vh -> vh
* vmin -> vmin
* vmax -> vmax
* pct -> %

All the units are functions and simply return a string representing the appropriate CSS value

## CSS Colors
Pbss supports CSS colors like _rgb_, _rgba_, _hsl_, _hsla_ natively. Let's declare the background and foreground color pf a div

``` python
from pbss import *

root = {
    "div": {
        "background": rgb(200,0,0),
        "color": rgba(0,255,0),
    }
}
```
This generates the following CSS
``` CSS
div {
    background: rgb(200,0,0);
    color: rgba(0,255,0);
}
```
Pbss supports the following color units natively
* RGB
* RGBA
* HSL
* HSLA

__Note hex values can be put in string form so no implementation has been done for it__  
__Also hex values can be generated from rgb colors by passing the True argument after the color numbers or by calling the _.to_hex()_ method__
