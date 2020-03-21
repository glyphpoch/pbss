# Pbss: Practically Better Style Sheets
Pbss is an attempt to bring Python to CSS. Pbss compiles Python code to CSS. So you get the features of Python and you don't deal with CSS as the code is converted to CSS by Pbss.

## Getting Started With Pbs- Get the latest release from Github and install using pip- Get the latest wheel file from Github and install using pip  
        pip install pbss_pbssteam-[VERSION_NO]-py3-none-any.whl
- Create you file containing Python code. Example:- Here we create a simple file test.py to reset body margin and padding along with setting font family to Arial.
``` python
root = {
    "body": {
        "padding": 0,
        "margin": 0,
        "font-family": "Arial"
    }
}
```
root is a dictionary that is required to be in the Python file that is parsed to get the selectors, their properties and values.  
- Run the following command
        python pbss test.py style.css
The above steps create a file called style.css and it contains the following code
``` CSS
body {
    padding: 0;
    margin: 0,
    font-family: Arial;
}
```

## Why Use Pbss over other programs like Sass/Less etc
Everyone knows that Python is easy and has tonnes of features. All of those features can be used in the root dictionary. Pbss supports nesting of elements (See docs), color arithmetic, supports CSS units natively intelligently detects whether its a pseudo-selector like _:hover_ and it supports continuous watching of file and compiling. Moreover Pbss supports many tools so that you don't have to put them as strings. For example all CSS units are natively supported in Pbss.

## Licensing and Price
Pbss is completely free for personal and commercial use and is Licensed under the open-source MIT License.
