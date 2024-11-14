<div align="center" id="top"> 
  <img src="./.github/app.gif" alt="ServerLauncher" />

  &#xa0;

  <!-- <a href="https://serverlauncher.netlify.app">Demo</a> -->
</div>

<h1 align="center">ServerLauncher</h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/xsxz01/ServerLauncher?color=56BEB8">

  <img alt="Github language count" src="https://img.shields.io/github/languages/count/xsxz01/ServerLauncher?color=56BEB8">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/xsxz01/ServerLauncher?color=56BEB8">

  <img alt="License" src="https://img.shields.io/github/license/xsxz01/ServerLauncher?color=56BEB8">

  <img alt="Github issues" src="https://img.shields.io/github/issues/xsxz01/ServerLauncher?color=56BEB8" />

  <img alt="Github forks" src="https://img.shields.io/github/forks/xsxz01/ServerLauncher?color=56BEB8" />

  <img alt="Github stars" src="https://img.shields.io/github/stars/xsxz01/ServerLauncher?color=56BEB8" />
</p>

<!-- Status -->

<!-- <h4 align="center"> 
	🚧  ServerLauncher 🚀 Under construction...  🚧
</h4> 

<hr> -->

<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0; 
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#rocket-technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/xsxz01" target="_blank">Author</a>
</p>

<br>

## :dart: About ##

A server launcher for starting and managing servers.

## :sparkles: Features ##

- Start and manage the server with a single command
- Support multiple servers
- [x] Custom startup commands
- [x] Custom stop commands
- Easy to use
- Use Rhai (A script as similar as Javascript with Lua) script to control the server
- [ ] More features to come...

## :rocket: Technologies ##

The following tools were used in this project:

- [Rust](https://www.rust-lang.org/)
- [Clap](https://github.com/clap-rs/clap)
- [Rhai](https://rhai.rs/)

## :white_check_mark: Requirements ##

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com) and [Node](https://nodejs.org/en/) installed.

## :checkered_flag: Install ##

1. Install the binary executable cli.

```bash
cargo install ServerLauncher
```

2. create the config file
```bash
mkdir  lua
touch lua/main.rhai
```

3. And put it in the config file `lua/main.rhai`

```rhai
fn start() {
    run_script("/data/BDZC/LogServer/", "start.sh", "");
}
fn stop() {
    run_script("/data/BDZC/LogServer/", "stop.sh", "");
}
```

> `start` method: start the server  
> `stop` method: stop the server  
> `run_script` method: run the script，it has three parameters, the first is the path of the script, the second is the script name, the third is the script parameters.

## :checkered_flag: Starting ##

```bash
# Clone this project
$ git clone https://github.com/xsxz01/ServerLauncher

# Access
$ cd ServerLauncher

# Build Binary Executable
$ cargo build

```

## :memo: License ##

This project is under license from MIT. For more details, see the [LICENSE](LICENSE) file.


Made with :heart: by <a href="https://github.com/xsxz01" target="_blank">Pang</a>

&#xa0;

<a href="#top">Back to top</a>
