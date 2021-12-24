# Gamepiler

The gamepiler *will* be a tool for checking your errors as achievements.

![tweet](tweet.png)


How it started ^^. At least 150 people liked a tool that would gamify their error as achievements (yay! celebrate errors), and 7 (inclusive meself) needed it very badly. The Gamepiler 🍍 idea was planted.

![compiler](compiler.png)


Right now it is just counting errors and saving them in a file BUUUUUT there is some hope to turn it into a real cargo tool that will give you stats.

Your error file will be in the `data local dir`, as per [the default paths from the directory crate](https://github.com/dirs-dev/directories-rs).

Atm, you need to download the repository, compile it with nightly:

`cargo +nightly install --path ./.`

You can then go in another directory and run 
`cargo-gamepile` to get a list of errors, or `cargo-gamepile --graph`for a graph which is incorrect atm (🙃) or `cargo-gamepile --xkcd` for an xkcd style graph that works.