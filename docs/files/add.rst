Attach: Join Files
==========================
How many times you wanted to break your style sheet into two or more files. CSS dose not support that unless you change your HTML. Well Pbss has you covered. We have a feature that attaches any number of files you like with just one line of code. ``attach()`` is a function that joins the root dictionary of any number of files. For example, we have two files
master.py and style.py with the following code in master.py

.. code-block::
    :linenos:

    root = {
        "html": {
            "font-family": "sans-serif"
        }
    }

The style.py contains the following code

.. code-block::
    :linenos:

    root = {
        "body": {
            "padding": 0,
            "margin": 0,
        }
    }

Now we want to join or merge these two dictionaries, well we use the ``attach`` method from pbss in style.py, if you use the syntax described in other pages, you will already have it but it is like this::

    from pbss import *

Assuming master.py is in the same folder as that of style.py, append this end of style.py add this line::

    root = attach(root, "master.py")

add takes the filename to be added and add it at the beginning of the ``root`` dict of the current file

Just for clearance style.py looks something like this

.. code-block::
    :linenos:

    from pbss import *

    root = {
        "body": {
            "padding": 0,
            "margin": 0,
        }
    }

    add(root, "master.py")

This generates the following code

.. code-block:: css
    :linenos:

    html {
        font-family: sans-serif;
    }
    body {
        padding: 0;
        margin: 0;
    }

You can give as many files as you like to ``attach`` and it will join
But maybe you want to reverse the order. Maybe you want that *body* should come before all other blocks, well *attach* can handle that too, just pass the ``placement`` argument to "b" after all files, like this::
    
    root = attach(root, "master.py", placement="b")

Now here's your stylesheet

.. code-block:: css
    :linenos:

    body {
        padding: 0;
        margin: 0;
    }
    html {
        font-family: sans-serif;
    }

This places the ``root`` at the very beginning. The default is value for placement is ``e``

**Note that the .py extension can be omitted in attach method**
