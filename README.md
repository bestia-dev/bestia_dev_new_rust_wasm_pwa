[comment]: # (auto_md_to_doc_comments segment start A)

# bestia_dev_new_rust_wasm_pwa

[comment]: # (auto_cargo_toml_to_md start)

**Creates a new Rust Wasm PWA project**  
***version: 2022.714.2131 date: 2022-07-14 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1709-green.svg)](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-76-blue.svg)](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-97-purple.svg)](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/)

[comment]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/blob/master/LICENSE)
[![Rust](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/)
![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/644173827.svg)

Hashtags: #rustlang #tutorial #pwa #wasm #webassembly  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Try it

<https://bestia.dev/bestia_dev_new_rust_wasm_pwa/>
 
![screenshot](https://github.com/bestia-dev/bestia_dev_new_rust_wasm_pwa/raw/main/images/screen_1.jpg)

## Motivation

I think PWA (Progressive Web Applications) are the future for many types of applications. In combination with Rust and Wasm it is a really great development platform that works almost everywhere.
But it is tricky to put all the scaffolding in place and start a new project.
So I created a simple application to create this initial scaffolding. It is like `cargo new`, but for Rust_Wasm_PWA.  
To allow input of parameters in a GUI I didn't create a CLI, but a PWA.  
Practice what you preach !

## Youtube video

I created a video on youtube to demonstrate how to use it.
TODO: image and link to youtube

## Development of the utility

I use `cargo-auto` to automate development tasks: `cargo install cargo-auto`.  
Then inside the Rust project folder `cargo auto` for the instructions.
PWA files MUST be served by a web server. We will use the most simple development web server: `cargo install basic-http-server`.  
Open a new terminal window in VSCode and go to the web server folder and run the server:  
`cd ~/rustprojects/bestia_dev_new_rust_wasm_pwa/web_server_folder; basic-http-server`  
Inside VSCode add the port 4000 for forwarding out of the docker container.
Open the browser in Win10 on:  
<http://127.0.0.1:4000/bestia_dev_new_rust_wasm_pwa/>  

## Video Subtitles

Welcome to bestia.dev !
Learning Rust and Wasm programming and having fun.
I just love  programming !

In my first video tutorial, we set up WSL 2 (Windows subsystem for Linux) with Debian 11 on Windows 10.
In the second video, we created a Docker container with a complete Rust development environment to use with VS Code.
In the third video, we created a simple CLI (command line interface) application to demonstrate how Rust development works in real life.

Today we will go a step further to perfection.  
The Rust code can be compiled for almost every architecture and Operating system. And the CLI interface for text terminals is almost universal.  
What we really want is to have a Graphical user interface, but this is a problem Rust cannot help us.  
Every Operating system invented a totally incompatible library for the G.U.I. On purpose, so they can sell expensive things and have a monopoly.
There are a few universal G.U.I libraries that works on many OS, but probably not everywhere.
I think that the only true world standard for G.U.I is HTML5 with CSS3. That will probably work everywhere.
It is not a light solution, the HTML and CSS must be rendered by a browser. But if the whole web is created around it, that must mean it is quite good.

The G.U.I must be manipulated programmatically and unfortunately for 25 years we were cursed to use only Javascript. That was not great at all.
Fortunately all browser manufacturers agreed and implemented WebAssembly or WASM. The best language to be compiled to WASM is Rust. Great for us Rustaceans!

Traditional Web pages and Web Applications needed an internet connection to be online to work. For a simple utility application we would like to use it offline.
Enter PWA (Progressive Web Application). This is a small standard that enables offline use of Web Applications and it is implemented in all modern browsers. Hooray!

This demonstrative project will be fairly simple, just to show the main scaffolding needed to make a PWA using WASM and Rust.

The problem we are solving is simple: Text sort.  
I have a list of song titles and I want to sort them alphabetically.
There are many, many solutions all around: online, inside text editors, as bash command,... but I need a special sorting that is not English.
Everything then falls apart ! Most of the solutions are English only. Or it is very complicated to select the desired collation (alphabetical order).

Let's get started!

After reboot I need to initialize my Rust development environment with a short script.
Then I can open VSCode press F1 and connect to the Remote SSH Host.
There I will create the directory for the new project.
The name of the project will be "Sort text international PWA in Rust WASM".

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.

Please, spread this info.

You can also read crev reviews quickly on the web:

<https://web.crev.dev/rust-reviews/crates/>

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).

I just love programming.

But I need also to drink. If you find my projects and tutorials helpful,please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).

You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)

[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
