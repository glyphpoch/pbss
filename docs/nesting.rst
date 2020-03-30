Nesting
=======
Pbss supports nesting which means selectors can be nested one inside the other similar to HTML. Let's write a code a *nav* selector with certain properties with a *ul* containing its own properties

.. code-block::
    :linenos:

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

On compiling this, the following code is generated

.. code-block:: css
    :linenos:

    nav {
        display: flex;
        position: fixed;
        top: 0;
    }
    nav ul {
        margin-left: auto;
    }

Nesting can be any level deep for example if you have *li* inside your *ul*, then you can change their properties too

.. code-block:: python
    :linenos:

    "ul": {
        "margin-left": "auto",
        "li": {
            "margin": 0
        }
    },

**Note: this is continuation from the above example**

The following code will be generated

.. code-block:: css
    :linenos:

    nav ul {
        margin-left: auto;
    }
    nav ul li {
        margin: 0;
    }
