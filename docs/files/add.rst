Add: Join two Files
==========================
``add()`` is a function that joins the root dictionary of two files. For example, we have two files
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

Now we want to join or merge these two dictionaries, well we use the add method from pbss in style.py, if you use the syntax described in other pages, you will already have it but it is like this::

    from pbss import *

Assuming master.py is in the same folder as that of style.py, append this end of style.py add this line::

    root = add("master.py")

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

**Note that the .py extension can be omitted in add method**
