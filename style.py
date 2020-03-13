from pbss.colors import rgb, rgba, als
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
            "font-family": copy("* font-family"),
            "background": rgb(50,50,50) + rgb(20,20,20),
            "a": {
                "color": rgba(100,100,100, 0.5) + als(20,0.5)
                }
        }
    }
}
