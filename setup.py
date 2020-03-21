import setuptools

with open("README.md") as r:
    desc = r.read()
    
setuptools.setup(
    name = "pbss",
    version = "1.0",
    author = "Arijit Dey",
    author_email = "arijid79@gmail.com",
    description = "Practically Better Stylesheets",
    long_description = desc,
    url = "https://github.com/arijid79/Pbss",
    packages = setuptools.find_packages(),
    classifiers = {
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent"
        }, 
    python_requires = ">=3.7"
)
