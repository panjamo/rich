use arboard::Clipboard;
use markdown::to_html;

fn main() {
    let mut clipboard = Clipboard::new().expect("Failed to open clipboard");

    // 1. Read the current clipboard text (Markdown)
    let md_input = match clipboard.get_text() {
        Ok(text) => text,
        Err(_) => {
            println!("Clipboard is empty or doesn't contain text.");
            return;
        }
    };

    // 2. Convert Markdown to HTML
    // We use 'to_html' with default options
    let html_output = to_html(&md_input);

    // 3. Set the clipboard as HTML
    // This function tells the OS "This is formatted text"
    // We provide the HTML version AND a plain text fallback
    match clipboard.set().html(html_output, Some(md_input)) {
        Ok(_) => println!("Success! Markdown converted to Rich Text."),
        Err(e) => eprintln!("Error setting clipboard: {}", e),
    }
}