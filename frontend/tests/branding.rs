// SPDX-License-Identifier: GPL-3.0-or-later

//! M5: two branding folders change the same compiled UI, including its slides.
use frontend::{AppWindow, Theme, apply_branding, branding};
use slint::{ComponentHandle, Model};
use std::path::Path;

fn folder(path: &Path, product: &str, color: &str, size: u32) {
    std::fs::create_dir_all(path.join("slides")).unwrap();
    std::fs::write(path.join("branding.toml"), format!(
        "product_name = \"{product}\"\nversion_string = \"test\"\naccent_color = \"{color}\"\nwebsite = \"https://example.org\"\nsupport_url = \"https://example.org/help\"\n"
    )).unwrap();
    let svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{size}\" height=\"{size}\"><rect width=\"{size}\" height=\"{size}\" fill=\"{color}\"/></svg>"
    );
    for name in ["logo.svg", "icon.svg"] {
        std::fs::write(path.join(name), &svg).unwrap();
    }
    std::fs::write(path.join("welcome.md"), product).unwrap();
    // Create in reverse order: filenames, not directory enumeration, order the slides.
    for n in ["02", "01"] {
        std::fs::write(
            path.join(format!("slides/{n}.png")),
            if product == "First system" {
                &include_bytes!("../../branding/luminos/slides/01.png")[..]
            } else {
                &include_bytes!("../../branding/luminos/slides/02.png")[..]
            },
        )
        .unwrap();
        std::fs::write(
            path.join(format!("slides/{n}.txt")),
            format!("{product} {n}"),
        )
        .unwrap();
    }
}

#[test]
fn swapping_folders_changes_the_same_compiled_window() {
    i_slint_backend_testing::init_no_event_loop();
    let root = std::env::temp_dir().join(format!("dawn-branding-{}", std::process::id()));
    let one = root.join("one");
    let two = root.join("two");
    folder(&one, "First system", "#123456", 24);
    folder(&two, "Second system", "#abcdef", 48);
    let app = AppWindow::new().unwrap();
    let mut first_slide = None;
    for (dir, name, size, red) in [
        (&one, "First system", 24, 0x12),
        (&two, "Second system", 48, 0xab),
    ] {
        apply_branding(&app, branding::load(dir).unwrap());
        let theme = app.global::<Theme>();
        assert_eq!(theme.get_product_name(), name);
        assert_eq!(theme.get_welcome_text(), name);
        assert_eq!(theme.get_logo().size().width, size);
        assert_eq!(theme.get_icon().size().width, size);
        assert_eq!(theme.get_accent().red(), red);
        let slides = theme.get_slides();
        assert_eq!(slides.row_count(), 2);
        assert_eq!(slides.row_data(0).unwrap().caption, format!("{name} 01"));
        assert_eq!(slides.row_data(1).unwrap().caption, format!("{name} 02"));
        let pixels = slides
            .row_data(0)
            .unwrap()
            .image
            .to_rgba8()
            .unwrap()
            .as_bytes()
            .to_vec();
        if let Some(ref first) = first_slide {
            assert_ne!(&pixels, first);
        } else {
            first_slide = Some(pixels);
        }
    }
    // The actual Installing screen rotates slides and pauses while reading logs.
    app.set_current_screen(7);
    app.show().unwrap();
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::ZERO);
    let visible = |caption: &str| {
        i_slint_backend_testing::ElementHandle::find_by_accessible_label(&app, caption)
            .next()
            .is_some()
    };
    assert!(visible("Second system 01"));
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_secs(9));
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::ZERO);
    assert!(visible("Second system 02"));
    app.set_show_install_log(true);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::ZERO);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_secs(16));
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::ZERO);
    app.set_show_install_log(false);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::ZERO);
    assert!(visible("Second system 02"));
    app.hide().unwrap();
    // No slides is valid: the progress screen works without a slideshow.
    std::fs::remove_dir_all(two.join("slides")).unwrap();
    assert!(branding::load(&two).unwrap().slides.is_empty());
    // A broken image must be reported instead of silently skipping branding.
    std::fs::create_dir(two.join("slides")).unwrap();
    std::fs::write(two.join("slides/99.png"), "not an image").unwrap();
    assert!(branding::load(&two).is_err());
    std::fs::remove_dir_all(root).unwrap();
}
