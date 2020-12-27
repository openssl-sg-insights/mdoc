# :page_facing_up: KODB - [kmaasrud's](https://github.com/kmaasrud) Opinionated Document Builder

This is my personal and (extremely) opinionated document builder script, built around my personal document creation workflow.

When doing scientific writing, I write using [pandoc](https://pandoc.org/) and it's flavor of Markdown. I absolutely adore Pandoc, because of it's immense flexibility and simplicity, that comes without losing the power of LaTeX (if I lack any features, I simply write some inline LaTeX, and it will be parsed perfectly). However, some kind of LaTeX engine is necessary, which usually defaults to the rather bloated and messy TeXLive :vomiting_face: To solve this, I use [Tectonic](https://tectonic-typesetting.github.io/en-US/), an excellent and modernized LaTeX engine built in C and Rust (it's much faster). I also need an intuitive Markdown syntax for cross-referencing and to handle bibliography. For this I have a couple of preferred Pandoc filters I regularly use.

In total, this leads to quite a convoluted process, and to keep my head clear, the project requires a strict and clean directory structure. `kodb` automates everything for me, and ensures a consistent structure and workflow for my scientific writing. This also facilitates easier collaboration.
