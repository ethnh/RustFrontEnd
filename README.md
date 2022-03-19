# Rust-based Frontend Example

This is a Rust-based frontend application made with [Yew](https://github.com/yewstack/yew), [Gloo](https://github.com/rustwasm/gloo), and [more](./Cargo.toml), compiled to [WebAssembly](https://www.arewewebyet.org/)

There is some CSS / HTML pulled from codepens, but everything else is written by me.

A Preview of this site can be found [here](https://ethanhindmarsh.com/snow/), or at this moment, here: [ethanhindmarsh.com](https://ethanhindmarsh.com/)

## Building

#### Development:
In development, it is recommended you use [Trunk](https://trunkrs.dev/) to watch the files for changes and automatically rebuild the application.

To do this, simply run `trunk sere` in the root of the project, and visit `http://localhost:8080` in your browser.

#### Production:
To build the application for production, run `trunk build --release`.