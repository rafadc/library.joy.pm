use std::fs;
use tinytemplate::TinyTemplate;
use crate::book_metadata::BookMetadata;
use serde::Serialize;
use slug::slugify;

static TEMPLATE : &str = "---\n\
title: {book.title}\n\
authors: [{book.authors | as_comma_separated_list}]\n\
openlibrary_id: {book.openlibrary_id}\n\
openlibrary_cover_edition_id: {book.openlibrary_cover_edition_id}\n\
openlibrary_author_ids: [{book.openlibrary_author_ids | as_comma_separated_list}]\n\
finished_at: {book.finished_at}\n
last_updated_at: {book.last_updated_at}\n
---\
";

#[derive(Serialize)]
pub struct TemplateContext<'a> {
    pub book: &'a BookMetadata,
    pub printable_authors: String,
    pub printable_author_ids: String,
}


pub fn generate_template(book: &BookMetadata) -> Result<(), std::io::Error> {
    let mut tt = TinyTemplate::new();
    tt.add_formatter("as_comma_separated_list", as_comma_separated_list);
    tt.add_template("book", TEMPLATE).unwrap();

    let context = TemplateContext {
        book,
        printable_authors: book.authors.join(","),
        printable_author_ids: book.openlibrary_author_ids.join(",")
    };

    let rendered = tt.render("book", &context).unwrap();

    let book_slug = slugify(&book.title);
    fs::write(format!("site/books/{}.md", book_slug), rendered).unwrap();

    Ok(())
}

fn as_comma_separated_list(value: &serde_json::value::Value, output: &mut String) -> tinytemplate::error::Result<()> {
    let values_as_strings: Vec<String> = value
        .as_array()
        .unwrap()
        .iter()
        .map(|x| format!("\"{}\"",x.as_str().unwrap().to_string()))
        .collect();
    output.push_str(values_as_strings.join(",").as_str());
    Ok(())
}