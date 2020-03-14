from pbss import copy, rgb, rgba, als, add
import master

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
root = add(root, master.root)
