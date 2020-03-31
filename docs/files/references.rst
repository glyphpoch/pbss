References
===========
.. module:: pbss

.. function:: add(base, readfile)

    Returns a dictionary  by joining the base ``root`` dictionary and the readfile ``root`` dictionary

    ::

        root = add(root, "filename.py")

    **OR**

    ::

        root = add(root, "filename")

    **Params:**
        ``base``
            The *root* dictionary of the module to which readfile
            should be joint
        ``readfile``
            The file path whose ``root`` dict is to be joint
            The file extension must be *.py* but can be omitted in argument

.. module:: colors

.. class:: rgb(red, green, blue[, is_hex=False])

    Returns a RGB object from the given *red*, *green*, and *blue*.
    Returns a hexadecimal object if is_hex is passed True

    ::

        rgb(0,155,286)

    **Params:**
        ``red``
            The red shade of the RGB color
        ``green``
            The green shade of the RGB color
        ``blue``
            The blue shade of the RGB color
        *Optional* ``is_hex``
            If the value is passed True, then a hex value is compiled
            instead of RGB

    .. function:: __str__()

        Returns a string RGB that should be used in actual style sheet

    .. function:: to_hex()

        Returns the hexadecimal representation of the RGB

        ::

            rgb(0,155,286).to_hex()

    .. function:: to_rgba([alpha=1])

        Returns the RGBA representation of the RGB

        ::

            rgb(0,155,286).to_rgba(0.5)

        **Params:**
            *Optional* ``alpha``
                The alpha that should be set for RGBA


.. class:: rgba(red, green, blue, alpha)

    Returns a RGBA object from the given *red*, *green*, and *blue* and *alpha*

    ::

        rgba(0,155,286)

    **Params:**
        ``red``
            The red shade of the RGBA color
        ``green``
            The green shade of the RGBA color
        ``blue``
            The blue shade of the RGBA color
        ``alpha``
            The alpha that should be set for RGBA

    .. function:: __str__()

        Returns a string RGBA that should be used in actual style sheet

    .. function:: to_rgb()

        Returns the RGB representation of the RGBA by removing alpha
        and creating new object

        ::

            rgba(0,155,286,0.5).to_rgb()


.. class:: hsl(hue, saturation, lightness)

    Returns a HSL object from the given *hue*, *saturation*, and *lightness*.

    ::

        hsl(5,10,15)

    **Params:**
        ``hue``
            The red shade of the HSL color
        ``saturation``
            The green shade of the HSL color
        ``lightness``
            The blue shade of the HSL color

    .. function:: __str__()

        Returns a string HSL that should be used in actual style sheet

    .. function:: to_hsla([alpha=1])

        Returns the HSLA representation of the HSL

        ::

            hsl(5,10,15).to_hsla(0.5)

        **Params:**
            *Optional* ``alpha``
                The alpha that should be set for RGBA


.. class:: hsla(hue, saturation, lightness, alpha)

    Returns a HSL object from the given *hue*, *saturation*, and *lightness*, and *alpha*.

    ::

        hsla(5,10,15,0.5)

    **Params:**
        ``hue``
            The red shade of the HSL color
        ``saturation``
            The green shade of the HSL color
        ``lightness``
            The blue shade of the HSL color
        ``alpha``
            The alpha that should be set for RGBA

    .. function:: __str__()

        Returns a string HSLA that should be used in actual style sheet

    .. function:: to_hsl()

        Returns the HSL representation of the HSLA by removing alpha
        and creating new object

        ::

            hsl(5,10,15,0.5).to_hsl()

.. module:: file

.. class:: File(fpath, fmod)

    Returns a File object from *fpath*. *fmod* tells whether it is
    for reading or writing, if reading, check if the file exists
    if not, then quit. Reading is denoted by r and writing by w

    ::

        File("filename.py", "r")

    .. function:: get_mod_time()

        Returns the most recent modification time of a file

        ::

            File("filename.py", "r").get_mod_time()

    .. function:: __str__()

        Return the path to the file

    .. function:: watch_file(func)

        Starts watching the file and execute *func* when a modification is done

        ::

            File("filename.py", "r").watch_file(func)

        **Params:**
            ``func``
                The function to be executed when a change occurs

.. module handle

.. class:: Main()

    The main class that is run by pbss to start the program

    .. function:: get_dict_css()

        Parses the *self.readfile* for the ``root`` dictionary and returns it

    .. function:: writer(content)

        Writes *content* to *self.writefile*

        **Params:**
            ``content``
                The text to be written

    .. function:: get_args(args)

        Parses *args*, if its empty then no hard-coded arguments ad
        arguments are taken from *sys.argv*. Sets *readfile* and *writefile*

    .. function:: recompile()
        Executes functions 
