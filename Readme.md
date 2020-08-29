# Competing standards

## Problem

<img src="https://imgs.xkcd.com/comics/standards.png" alt="competing standards" />

For reasons both JIRA and Confluence have their own flavour of markdown, incompatible with spec. Whereas it is possible to generate plain html from markdown and insert into confluence page, it is not an option for JIRA tickets. Which is unfortunate if one prefers to write their notes in md as they explore the task/codebase. 

This simple utility translates markdown docs into format understood by JIRA. 

## Install 

- [install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- `git clone`
- `cargo install --path .`

### Nix

`nix run github:lainera/jirlate -- some-markdown-file.md`

## Usage

```
jirlate some-markdown-file.md
```

