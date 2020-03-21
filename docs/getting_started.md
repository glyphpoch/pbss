# Getting Started

Let's create a simple Python file that resets the margins and paddings of the _body_ element in Python

* Create a python file in your prefered text editor with any filename. We will create a file style.py for example
* Now write the following code
``` python
root = {
    "body": {
        "padding": 0,
        "margin": 0,
    }
}
```
___Root___ is a dictionary that is parsed by Pbss to get the selectors, properties and values

* The above can't be understood by any web browser as CSS so it needs to be compiled. So run the following command in terminal or command prompt
```
$ python -m pbss style.py style.css
Compiled style.py and wrote to style.css
```
This creates a file _style.css_ with the following content
``` CSS
body {
    padding: 0;
    margin: 0;
}
```
* If you want your code should be compiled as soon as you save your Python file, you should put the _-w_ option to pbss like this
```
python -m pbss -w style.py style.css
Compiled style.py and wrote to style.css
```
This would continuously watch for changes on the Python file and compile as soon as the file changes
