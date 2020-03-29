import time, sys

def watch_file(fobj, func):
    last_mod = fobj.get_mod_time()
    while True:
        try:
            c_time = fobj.get_mod_time()
            if last_mod == c_time:
                time.sleep(1)
            else:
                func()
                last_mod = c_time
        except KeyboardInterrupt:
            sys.exit(0)
        except Exception as excep:
            print(excep)
            last_mod = c_time
