use arboard::Clipboard;
use pulldown_cmark::{html, Options, Parser};
use std::io::{self, IsTerminal, Read};

fn main() {
    let mut clipboard = Clipboard::new().expect("Failed to open clipboard");

    // 1. Read Markdown from STDIN if piped, otherwise from clipboard
    let md_input = if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from STDIN");
        buffer
    } else {
        match clipboard.get_text() {
            Ok(text) => text,
            Err(_) => {
                println!("Clipboard is empty or doesn't contain text.");
                return;
            }
        }
    };

    // 2. Convert Markdown to HTML with table support
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&md_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 3. Set the clipboard as HTML
    // This function tells the OS "This is formatted text"
    // We provide the HTML version AND a plain text fallback
    match clipboard.set().html(html_output, Some(md_input)) {
        Ok(_) => println!("Success! Markdown converted to Rich Text."),
        Err(e) => eprintln!("Error setting clipboard: {}", e),
    }
}