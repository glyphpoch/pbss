Installing Pbss
===============
Before using Pbss, you need to install it in your computer. There are different methods of installation

.. highlight:: console

Getting Stable Releases
************************
Recommended Method (Windows, Most \*NIX and Mac OS)
###################################################
#. Download the latest *.whl* from the `Releases <https://github.com/arijit79/Pbss/releases>`_ page in whatever folder you like

#. Now open a Command Prompt/Terminal and navigate to the folder where you kept the *.whl* file. The file should have a nmae like
        pbss-[VERSION_NO]-py3-none-any.whl

#. Write the following command::

    pip install pbss-[VERSION_NO]-py3-none-any.whl

#. Once the installation completes, you can check its version by typing::

    pbss -v

AUR Package (Arch Linux)
########################
**Note: This process is only for users of Arch Linux**
If you have a AUR helper like *yay* or *yaourt*, you need to type the following command in the terminal::

    yay -S pbss

Or for yaourt::

    yaourt -S pbss

Getting Snap and Nightly Builds
********************************
These are our branches where you can get the most bleeding edge features. The snap release is a release made every 15 days that is sent out for those who want to try new features. The nightly build is updated on a daily basis with new features. The snap release might be somewhat usable but we do not guarantee as that of nightly builds

Recommended Method (Windows, Most \*NIX and Mac OS)
###################################################
#. Clone the repository using the following command::

    git clone https://github.com/arijit79/Pbss.git

#. Checkout the desired branch. Snap is refered by snap and the nightly is refered by dev::

    git checkout [branch-name]

#. Type the following command::

    pip install .

Maintaining the branch with updates
____________________________________
Whenever you want to get the new changes brought by us on these branches, use this command::

    git pull

Uninstalling Pbss
******************
To uninstall Pbss, type this command in the terminal::
    
    pip uninstall pbss