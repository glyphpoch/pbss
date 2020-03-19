from pbss import *
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
                "height": vh(15),
                "color": rgba(100,100,100, 0.5) + als(20,0.5)
            }
        }
    }, 
    "@media only screen and (max-width: 768px)": {
        "nav": {
            "padding": 0, 
        }
    }
}

root = add(root, "master")
