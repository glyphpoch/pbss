from pbss import rgb, rgba, als, add

root = {
	"*": {
		"font-family": "Arial, sans-serif"
	},
	"body": {
        "padding": 0,
        "background": rgb(20,20,20),
        "nav": {
            "background": rgb(50,50,50) + rgb(20,20,20),
            "a": {
                "color": rgba(100,100,100, 0.5) + als(20,0.5)
                }
        }
    }
}
root = add(root, "master")
