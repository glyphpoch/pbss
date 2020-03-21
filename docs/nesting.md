# Nesting
Pbss supports nesting which means selectors can be nested one inside the other similar to HTML. Let's write a code a _nav_ selector with certain properties with a _ul_ containing its own properties
``` python
root = {
    "nav": {
        "display": "flex",
        "ul": {
            "margin-left": "auto",
        },
        "position": "fixed",
        "top": 0
    }
}
```
On compiling this, the following code is generated
``` CSS
nav {
    display: flex;
    position: fixed;
    top: 0;
}
nav ul {
    margin-left: auto;
}
```

Nesting can be any level deep for example if you have _li_ inside your _ul_, then you can change their properties too
``` python
        "ul": {
            "margin-left": "auto",
            "li": {
                "margin": 0
            }
        },
```
__Note: this is continuation from the above example__  
The following code will be generated
``` CSS
nav ul li {
    margin: 0;
}
```
