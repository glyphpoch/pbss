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

.. code-block::
    :linenos:

    root = {
        "section": {
            "display": "grid",
            "grid-template-columns": rept(fr(1), 4)
        }
    }

The ``rept()`` function will first make a string of ``fr(1)``. Since all CSS units supported by Pbss return a string, so it will not affect it. Then it will return a string with 4 times of ``fr(1)``