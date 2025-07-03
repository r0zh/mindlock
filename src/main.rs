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
    window.set_default_size(400, 300);

    // Create a vertical box to hold both text and image
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    
    // Add the label
    let label = gtk4::Label::new(Some("Hello from the new window!"));
    vbox.append(&label);
    
    // Add a loading label for the image
    let loading_label = gtk4::Label::new(Some("Loading image..."));
    vbox.append(&loading_label);
    
    window.set_child(Some(&vbox));
    window.present();
    
    // Clone references for the async closure
    let vbox_clone = vbox.clone();
    let loading_label_clone = loading_label.clone();
    
    // Download and display the image asynchronously
    glib::spawn_future_local(async move {
        download_and_display_image(&vbox_clone, &loading_label_clone).await;
    });
}

async fn download_and_display_image(vbox: &gtk4::Box, loading_label: &gtk4::Label) {
    let image_url = "https://t4.ftcdn.net/jpg/01/29/74/59/360_F_129745961_J6Ok1s791kKBzKolxSjX4qhnNd7NIG4R.jpg";
    
    match reqwest::get(image_url).await {
        Ok(response) => {
            match response.bytes().await {
                Ok(image_data) => {
                    // Remove loading label
                    vbox.remove(loading_label);
                    
                    // Create image from bytes
                    let pixbuf = gdk_pixbuf::Pixbuf::from_stream(
                        &gio::MemoryInputStream::from_bytes(&glib::Bytes::from(&image_data)),
                        gio::Cancellable::NONE
                    );
                    
                    match pixbuf {
                        Ok(pixbuf) => {
                            // Scale the image to a reasonable size
                            let scaled_pixbuf = pixbuf.scale_simple(300, 200, gdk_pixbuf::InterpType::Bilinear);
                            
                            if let Some(scaled_pixbuf) = scaled_pixbuf {
                                let image = gtk4::Image::from_pixbuf(Some(&scaled_pixbuf));
                                vbox.append(&image);
                            } else {
                                let error_label = gtk4::Label::new(Some("Failed to scale image"));
                                vbox.append(&error_label);
                            }
                        }
                        Err(_) => {
                            let error_label = gtk4::Label::new(Some("Failed to load image"));
                            vbox.append(&error_label);
                        }
                    }
                }
                Err(_) => {
                    vbox.remove(loading_label);
                    let error_label = gtk4::Label::new(Some("Failed to download image"));
                    vbox.append(&error_label);
                }
            }
        }
        Err(_) => {
            vbox.remove(loading_label);
            let error_label = gtk4::Label::new(Some("Failed to fetch image"));
            vbox.append(&error_label);
        }
    }
}
