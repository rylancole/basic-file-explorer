# Basic File Explorer

## Getting Started

### Requirements

This project requires [Rust](https://www.rust-lang.org/tools/install), [Node.js](https://nodejs.org/en/download/), and the [Vue CLI](https://cli.vuejs.org/guide/installation.html) to run.

### Clone and Initialize

```
git clone https://github.com/rylancole/basic-file-explorer
cd basic-file-explorer
npm i
```

### Run

```
npm start
```

This runs multiple commmands to get the project started so you don't have to:

```
%> cargo fmt --manifest-path ./src-tauri/Cargo.toml
%> npx prettier --write src
%> vue-cli-service tauri:serve
```

## Motivation

I started this project in order to learn the following:

- Cross-Platform Desktop App Development
- Rust Programming Language
- Vue JavaScript Framework
- Code Auditing

### Cross-Platform Desktop App Development

I am personally very familiar with Mac OS and the Unix/Bash/Zsh enviroment, but I would like to be able to access users across all major platforms with my projects. For the Basic File Explorer I use [Tauri](https://tauri.studio/en/) to accomplish this, a toolkit to "build smaller, faster, and more secure desktop applications with a web frontend".

### Rust Programming Language

[According to Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language)), "Rust has been voted the "most loved programming language" in the Stack Overflow Developer Survey every year since 2016." This is what made the language catch my eye, and further reading has only made me more interested. I am building the back-end of the Basic File Explorer with [Rust](https://www.rust-lang.org/) to learn about the language and to gain familiarity with the types of problems it is good at solving.

### Vue JavaScript Framework

I have experience in [React.js](https://reactjs.org/) and would like to broaden my knowledge with other frameworks. I chose [Vue.js](https://vuejs.org/) because it is self-described as lightweight, specifically I chose the newly released Vue 3.0 so I can keep up with modern standards as well as leverage [TypeScript](https://www.typescriptlang.org/). 

### Code Auditing

I have a lot of interest in Consumer Privacy online and Open Source Software. One area that intruiges me is the auditing of code to ensure ethical standards of developers. I will use this project to explore the foundations of auditing a project to se if this is a path I would like to follow.

## TODO

### File Manipulation

1. Move files and folders
2. Delete files and folders
3. Rename files and folders
4. Create folders
5. Open files

### Settings

1. Set-up default folder on open
2. Edit sidebar 
  a. Add folder to sidebar
  b. remove folder from sidebar

### Cross-Platform

1. Create compatibility with Linux
2. Create compatibility with Windows




