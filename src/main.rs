use gtk4::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk4::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let button = gtk4::Button::with_label("Click me!");

    window.set_child(Some(&button));

    window.present();

    button.connect_clicked(|_| {
        // Create a new window when the button is clicked
        on_button_clicked();
    });
}

// spawn a new window on button click
fn on_button_clicked() {
    let window = gtk4::Window::new();
    window.set_title(Some("New Window"));
    window.set_default_size(200, 100);

    let label = gtk4::Label::new(Some("Hello from the new window!"));
    window.set_child(Some(&label));

    window.present();
}
