Repeating Objects
==================
Sometimes you want to write text which is just the repeatation of a few characters. For example consider this example

.. code-block:: python
    :linenos:

    root = {
        "section": {
            "display": "grid",
            "grid-template-columns": "1fr 1fr 1fr 1fr"
        }
    }

So we have improved this by introducing a new function called ``rept()``. Let's write the above code with this function.
First, as usual to get any Pbss's native functionality, you need to do the import::

    from pbss import *

Now let's write the above code. We will also make use of the new ``fr`` unit added to Pbss

.. code-block:: python
    :linenos:

    root = {
        "section": {
            "display": "grid",
            "grid-template-columns": rept(4, fr(1))
        }
    }

The ``rept()`` function will first make a string of ``fr(1)``. Since all CSS units supported by Pbss return a string, so it will not affect it. Then it will return a string with 4 times of ``fr(1)``. So this stylesheet will be generated

.. code-block:: css
    :linenos:

    section {
        display: grid;
        grid-template-columns: 1fr 2fr 1fr 2fr;
    }

It is also very smart in itself, for example say you want to write something like this::

    "grid-template-columns": "1fr 2fr 1fr 2fr"

Well you can put another argument after ``fr(1)`` that represent *2fr* like this::
    
    "grid-template-columns": rept(4, fr(1), fr(2))

This will generate this code

.. code-block:: css
    :linenos:

    section {
        display: grid;
        grid-template-columns: 1fr 2fr 1fr 2fr;
    }

You can also say rept to use delimiters to separate the above objects by passing the optional ``delimiter`` keyword argument. By default this is set to a space " ". Let's take the above code sample and put a comma as delimiter

.. code-block:: python
    :linenos:

    root = {
        "section": {
            "display": "grid",
            "grid-template-columns": rept(4, fr(1), delimiter=", ")
        }
    }

This will generate this code

.. code-block:: css
    :linenos:

    section {
        display: grid;
        grid-template-columns: 1fr, 2fr, 1fr, 2fr;
    }

**Notice that we use a space after comma, this is to ensure that the commas don't stick with the words**