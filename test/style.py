from pbss import *

root = {
    "body": {
        "padding": 0,
        "margin": 0,
    },
    "div": {
        "height": vh(10),
        "width": em(50)
    },
    "@media only screen and (max-width: 768px)": {
        "div": {
            "height": vh(50),
            "width": em(10),
        }
    }
}

root = attach(root, "test/master", placement="b")