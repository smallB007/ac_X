use cursive::views::{Dialog, TextView};
fn callback(app:&mut cursive::Cursive)
{
    Dialog::around(TextView::new("Some text"));
}
fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
                         .title("Cursive")
                         .button("Quit", |s| callback(s)));

    // Starts the event loop.
    siv.run();
}