use anyhow::Error;
use fehler::throws;
use termimad::MadSkin;

macro_rules! load_template {
    ($file:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $file))
    };
}

#[throws]
pub(crate) fn howto() {
    show_howto();
}

pub(crate) fn show_howto() {
    let markdown_input = load_template!("/src/howto.md");

    // Create a MadSkin for styling the Markdown.
    let skin = MadSkin::default();

    // Print the markdown content to the terminal.
    skin.print_text(markdown_input);
}
