# Contributing To Pbss and Flow of Releases
## Contributing
There are many ways you can contribute to Pbss. Here are some of the ways
### Writing Code
Pbss's development will be greatly helped if you help in writing code. This not only helps us, but to you as well as everyone using Pbss because the feature that you add will not only be used by you but by everyone.  
* First fork the [repository](https://gitlab.com/arijitd/pbss.git).
* Now make a separate branch where you write your code. Make sure that you rebase that from base it from the _dev_ branch.
* Open Gitlab and submit a pull request to the [issue tracker](https://gitlab.com/arijitd/pbss/-/issues), your code will be accepted by us in a short interval.

Some basic things to keep in mind while you write code :-
* The code should be formatted using rust format standards. You may use the __rustfmt__ or __cargo-fmt__
* If you write your code in the _dev_ or _snap_ branches, the code might not be accepted by us because it takes time to merge code between same branches written by us and the community especially if it requires manual intervention.
* You should also try to give us a basic documentation. At least very basic description of functions and structs should be provided along with workings and examples.
* You should not modify too many of the base functions and structs, rather you can write the function and send to us to implement it.

### Writing documentation
Another way to help us is by writing the documentation for us. This will help us write more code and cooler features for you. You can write documentation in the same way as for writing code. follow the same steps as above of forking and submitting pull request.  

Things to remember while writing docs :-
* If you are trying to describe a feature, then the file must go in the [docs](docs/) directory or folder.
* If you are writing docs for Pbss like what Pbss features, how to install it basically things that are directly to Pbss, you can write them in the README.md.
* If its a feature, you must give examples with it.
* You should always keep in mind to be as exact, to the point and as detailed as possible without taking too many lines. Because everyone loves short and to the point docs.

### Filing Bug Reports
It really helps if you help in finding bug reports so that we can fix it as quickly as possible. To file a bug report just simply report it to the [issue tracker](https://gitlab.com/arijitd/pbss/-/issues). You may also write fix for it if you like for us and follow the above instruction of writing code so that we may include it.

### Helping Others
The developers are generally busy writing code and so they do not get time to interact and help the people and community out. You can help people in the issues page or at other forums and websites like Stack Overflow

### Donating Us
Even though Pbss is completely free to use but it does require us to pay up the developmental charges. You can support us by simply giving us a few bucks of your livelihood. This helps a long way out. To donate to us, please send a reply to _arijid79@gmail.com_

## Flow of Releases
Pbss has a simple flow of releases. But before discussing that, we need to discuss Pbss's versioning scheme. Pbss's versions have a __v__ at front and then at number with like this __mm.ss__, where _mm_ stands for major release and _ss_ stands for snap release.
* Every two months we will have a major release that will increase the major release number by 1 and will reset the snap number to 0. Here we introduce major changes and feature updates for Pbss. The release will have a codename with it. We will also push out the wheel file needed for installing Pbss in this release

* Every 15 days a snap will be released in the snap branch, this will increase the snap release number by 1 but will not affect major release number. Here we give a snapshot of the development of Pbss to the community

* Every 3rd snap release will be a beta release for the next major release. Here we try to put out a mostly stable version but it may contain bugs

### Describing Branches
Pbss has three branches and no more branches will created ever
* __Master__: This branch holds the current and most stable build of Pbss. The _.whl_ files are generated from this branch. All users are recommended to use this release. This is updated on a 2 months release cycle

* __Snap__: This branch holds the current snapshot release of Pbss. You are not provided with any _.whl_ files for this branch and release and users need to clone and manually build and install Pbss. This is intended for those who want to try out the most cutting edge features. But this also contains the bugs and errors and their updates don't come until the next snap release as they are released as it is. This is updated every 15 days

* __Dev__: This branch is the nightly build of Pbss and should only be used by Contributors and making pull requests. We do not take any guarantee if the code in this branch works

If you try the snap or dev branch and found a bug, you should check if a issue exist on Gitlab, if it doesn't then you should file one.
