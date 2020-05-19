# Welcome to Pbss Documentation!
This is the official documentation for Pbss and has been rewritten for Pbss's second version from the ground up. The older documentation has been depricated and no longer available for viewing or use. The below table lists all major links to this document

**Table of Contents**
1. [Overview](#overview)
2. [What's new in Pbss 2.0](#release-info)
3. [Installation](#install)
4. [Parts of Pbss](#parts)
5. [Feature Explanantions](#explainations)


## <a name="overview"></a> 1. What is Pbss
Pbss is an attempt to improve on CSS 3 by providing better support for variables, breaking code into multiple files, error catching and lots of other features so that our users never feel the old boring CSS. More effectively said it is a preprocessor for CSS.  
Pbss is written in hte Rust programming language and is completely free to use for any purpose and licensed under the very permissive MIT License

## <a name="release-info"></a> 2. What's new in Pbss 2.0
Pbss 2.0 is the greatest release in the history of Pbss, since it had been rewritten in Rust. Prior to Pbss 1.2, Pbss was written in Python but was replaced with Rust due to its memory safty, better concurrency features, speed and robustness. Although the release did not supported all the features after it was started in rewriting but it did catched up in this release. It gave robust support for the following features
- Variables
- Dividing the source code into multiple files
- It made the later stages of developemnt much easier
- It no longer depends on the Python dictionary format to get the stylesheet
- It used multiple threads for better efficiency
- It provided easy error catching and wrote it to terminal in nicde manner

Although it lacked the features that its predecessors had brought to the table like
- It did not supported color arithmetic
- It cannot perform calculations
- It did not provide native support for colors
- It did not provide support for functions

## <a name="install"></a>3. Installation
Pbss is quite simple to install. Go to the releases page and download the executable for your OS. For Windows, it will have a *.exe* extension, Linux packages will need to get the *.tar.gz* file and for Mac Os

## <a name="parts"></a>4. Parts of Pbss
Pbss although has only one executabes and does not require any external libraries, has been affectionately been divided into two parts:-
* **Pbss:** which is the actual executable and frontend
* **Mynk:** the parser and compiler and writer engine. Mynk goes through the file line by line, classifies the type of line and performs actions that need to be taken for the line and report errors, if the line is considered invalid

## <a name="explainations"></a>5. Features Explainations
### Traditional CSS
CSS is written in the same way as you have always been writing, for example let's change the background color of body, set the width of all div tags to `100px` and increaes the font size to `2em`

``` CSS
* {
    font-size: 2em; /* All elements have a font size of 2em */
}
body {
    background: rgb(255,255,255); /* White background */
}
div {
    width: 100px; /* Height of all divs will be 100px */
}
```
