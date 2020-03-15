"""
This module should be run from the python interpreter

python -m pbss
"""

#!/usr/bin/env python
from .handle import Main

if __name__ == "__main__":
    Main().execute()
else:
    print("This program is only meant for execution")
