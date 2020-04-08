Installation
=============

.. contents:: Table Of Contents
    :backlinks: none

So before we get our hands dirty with the code let's first take care of what we need to install. For this tutorial, we expect you use any one of the following operating systems

* Windows
* Mac OS
* Ubuntu/Ubuntu Derivatives
* Debian
* RHEL/Fedora/Cent OS
* Arch Linux

**If you don't use any of the above OSs, then you may have to look up informations related to your operating system on covering the upcoming steps**

Let's see what are the basic requirements to use Pbss

* Python (3.7 or greater) and Pip
* Pbss
* A web browser (Firefox recommended since it has better CSS tools)
* A text editor (Sublime Text, Atom, VS code)
* A command line (Command Prompt/Terminal/PowerShell)

Getting Python
--------------
Installing On Windows
______________________
* Go to `Python Releases For Windows <https://www.python.org/downloads/windows>`_
* Download the *.exe* file for your computer's architecture
* Go through the installation to install Python
* Make sure that you check to install *pip* and add it to path

Installing on Ubuntu/Ubuntu Derivatives
_______________________________________
* Open a terminal
* Run this command to install Python 3 as well as Pip::

    sudo apt install python3 python3-pip

* Enter your password and press ENTER. Confirm the confirmation ny pressing ENTER

Installing on Debian
____________________
* Open a terminal
* Run this command::

    sudo apt-get install python3 python3-pip

* Enter your password and press ENTER. Confirm the confirmation ny pressing ENTER

Installing on RHEL/Fedora/Cent OS
_________________________________
* Open a terminal
* Run this command::

    sudo dnf install python3 python3-pip

* Alternatively you can use *yum*::

    sudo yum install python3 python3-pip

* Enter your password and press ENTER. Confirm the confirmation ny pressing ENTER

Arch Linux
__________
* Open a terminal
* Run this command::

    sudo pacman -S python python-pip

* Enter your password and press ENTER. Confirm the confirmation ny pressing ENTER

Getting Pbss
--------------
* Go to `Pbss releases Page <https://github.com/arijit79/Pbss/releases>`_
* Download the latest stable *.whl* file and place it wherever you like
* Now open a Terminal/Command Prompt and type the following command::
    
    pip install pbss-[VERSION-NO]-py3-none-any.whl

* Replace [VERSION-NO] with whatever version you downloaded
* Check it by running the following command::
    
    pbss -v

* It should return the Pbss version code along with Python version and GCC version

The other requirements are out of the scope of this tutorial. Therefore they should be installed by users by themselves.