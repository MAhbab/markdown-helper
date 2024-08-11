# Markdown Helper
This project enables working with markdown files using a command line interface. More specifically, it aims to allow for modifying and searching for markdown files that use [Markdown Frontmatter](https://frontmatter.codes/docs/markdown).

## Motivation
I, like many others, am a huge fan of [Obsidian](https://obsidian.md/) for note taking. However, Obsidian has two major deficiencies which prevent it from being the **perfect** note taking tool:

- **It is closed source**: Although Obsidian puts itself far ahead of competitors like [Notion](https://www.notion.so/) by using using local Markdown files over a proprietary data format stored in the cloud, the fact that it's closed source presents a persistent risk. I seek a note-taking application that can persist over years and possibly decades.
- **It lacks a native command line interface**: Obsidian's GUI is phenomenal, from its VS code-like command palette to its native Vim keybindings. That being said, there's some additional functionality I'd like that would fit best in a command line interface.

The goal of this project is a CLI and eventually (if I am not distracted by something else) a GUI that is compatible with Obsidian, replicates its core functionality, and addresses the two shortcomings listed above.

## Usage
Currently, there's only one command which summarizes a markdown file. You can call it from within this project's root directory like so:
```bash
cargo run -- my_markdown_file.md

#prints
#            File path: /tmp/notes/my_markdown_file.md
#            Timestamp: 2024-07-23T17:20:27Z,
#            Tags: print, tags, here,
#            Word Count: 410
```        
