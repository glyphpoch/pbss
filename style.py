from pbss.colors import rgb
from pbss import copy

root = {
	"*": {
		"font-family": "Arial, sans-serif"
	},
	"body": {
        "padding": 0,
        "margin": copy("padding"),
        "background": rgb(20,20,20),
        "nav": {
            "color": copy("/background"),
            "font-family": copy("* font-family")
        }
    }
}
