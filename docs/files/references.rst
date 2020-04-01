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

.. function:: rept(obj, num)

    Returns a string by multiplying ``obj`` with num. Note object can be of any type. It will be converted to string by the function

    ::

        rept(em(1), 4)

    **Params:**
        ``obj``
            Any object that will be converted to string and multiplied
        ``num``
            Number of times the object has to be multiplied

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

        Returns the :class:`RGBA <rgba>` representation of the RGB

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

        Returns the :class:`RGB <rgb>` representation of the RGBA by removing alpha
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

        Returns the :class:`HSLA <hsla>` representation of the HSL

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

        Returns the :class:`HSL <hsl>` representation of the HSLA by removing alpha
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

        Executes functions step by step starting from :func:`get_dict_css`, then run the :class:`parser.Parser` and finally writing the content via the :func:`writer` function

    .. function:: execute()

        This function is executed by the pbss to :func:`parse args <get_args>`, :func:`recompile <recompile>` and start :func:`watch mode <file.File.watch_file>` if requested

.. module:: parser

.. class:: Parser

    This contains all the methods for parsing the dictionary. It handles nesting,
    pseudo selectors and @ rules

    .. classmethod:: check_pseudo_selector(i, string)

        Check if i starts with *:*, if yes then remove a space from end of ``string`` and join ``string`` with ``i``

        **Params:**
            ``i``
                The actual selector on which to test
            ``string``
                The string on which if i passes, the last space is removed

    .. function:: at_ops(sel, base):

        Handles media queries and other @ rules, if the selector starts with @
        then parse all its inside dictionaries using :func:`get_properties` and also :func:`check nests <check_nests>`

        **Params:**
            ``sel``
                The actual @ rule on which to test
            ``base``
                The ``root`` dictionary

    .. function:: check_nests(nests, base)

        Checks if the nests contains elements, if so there are nested elements in the most recent dictionary parsed inside base and therefore runs :func:`get_properties` on each of them

        **Params:**
            ``nests``
                A list of lists containing the path to the nested element
            ``base``
                The ``root`` dictionary

    .. function:: get_properties(sel, base)

        The actual engine that parses dictionaries. First navigate to the path specified in ``sel`` list then parse all the keys and values one by one, if any value is instance of dict then copy its path and add it to nests, else format it in the CSS format like this::

                    Key: Value;

        Finally return the generated string and nests

        **Params:**
            ``sel``
                A path in form of list to navigate for properties
            ``base``
                The ``root`` dictionary


    .. function:: get_content()

        Returns the contents finally generated

.. module:: units

.. function:: cm(num)

    Returns num in centimeters

    ::

        cm(10)

    **Params**:
        ``num``: The number to be converted

.. function:: mm(num)

    Returns num in millimeters

    ::

        mm(10)

    **Params**:
        ``num``: The number to be converted

.. function:: inc(num)

    Returns num in inches

    ::

        inc(10)

    **Params**:
        ``num``: The number to be converted]

.. function:: px(num)

    Returns num in pixels

    ::

        px(10)

    **Params**:
        ``num``: The number to be converted

.. function:: pt(num)

    Returns num in points

    ::

        pt(10)

    **Params**:
        ``num``: The number to be converted

.. function:: pc(num)

    Returns num in pc

    ::

        pc(10)

    **Params**:
        ``num``: The number to be converted

.. function:: em(num)

    Returns num in em

    ::

        em(10)

    **Params**:
        ``num``: The number to be converted

.. function:: ex(num)

    Returns num in ex

    ::

        ex(10)

    **Params**:
        ``num``: The number to be converted

.. function:: ch(num)

    Returns num in xh

    ::

        ch(10)

    **Params**:
        ``num``: The number to be converted

.. function:: rem(num)

    Returns num in rem

    ::

        rem(10)

    **Params**:
        ``num``: The number to be converted

.. function:: vw(num)

    Returns num in vw

    ::

        vw(10)

    **Params**:
        ``num``: The number to be converted

.. function:: vh(num)

    Returns num in vh

    ::

        vh(10)

    **Params**:
        ``num``: The number to be converted

.. function:: vmin(num)

    Returns num in vmin

    ::

        vmin(10)

    **Params**:
        ``num``: The number to be converted

.. function:: vmax(num)

    Returns num in vmax

    ::

        vmax(10)

    **Params**:
        ``num``: The number to be converted

.. function:: pct(num)

    Returns num in %

    ::

        pct(10)

    **Params**:
        ``num``: The number to be converted

.. function:: fr(num)

    Returns num in fr

    ::

        fr(10)

    **Params**:
        ``num``: The number to be converted