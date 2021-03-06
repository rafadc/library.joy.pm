# Creator of templates for new books

A simple CLI to create a new template to retrieve metadata from a book. It downloads images for the cover and the author.

## Usage

It is expected to be run from library.cyg.us root folder. It looks for some relative paths at the moment.

```shell
post_template 0.1.0

USAGE:
    post_template [MD_FILE_TO_CONSOLIDATE]

ARGS:
    <MD_FILE_TO_CONSOLIDATE>    An existing book so we download again its images

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

We can start the command with no parameters and a UI to create a new template will be shown.

We can also choose an already existing markdown file so we can reprocess again all metadata we need for the site.

## TODO

* [ ] Exit with CTRL-C
* [ ] Probably I can serialize the markdown easily with serde straight on instead of using tiny template
* [ ] Be able to touch book to update last updated field
* [ ] Show the synopsis of a book
* [ ] Show a "searching" dialog when blocked by search
* [ ] Parameter for searching books in another language
* [ ] Parameter option to update metadata on an existing md
* [ ] Tags on books
