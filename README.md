# Blog source files

This repo contains the source files used by the blog engine [ecoweb](https://github.com/litchipi/ecoweb) to work.

All the content is licensed under a **Creative commons CC BY-NC** license.

#### Ecoweb configuration

Ecoweb expects a `config.toml` file, from this every other path can be modified as you wish.

This source dir is meant to be run with ecoweb using the `local_storage` feature, as posts are stored here in Markdown format.

## Posts registry

`registry.toml` and `series.toml` stores the metadata about the blog posts, and series of posts.

Because the blog will not host thousands of entries, it's the best solution for simplicity's sake.
