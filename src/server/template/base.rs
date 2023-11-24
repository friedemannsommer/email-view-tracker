use std::fmt::Write;

#[derive(Debug, Copy, Clone)]
pub enum ThemeColor {
    Danger,
    Dark,
    Primary,
    Success,
}

#[derive(Debug, Copy, Clone)]
pub enum ButtonType {
    Button,
    Submit,
}

markup::define! {
    Layout<'title, Header: markup::Render, Body:markup::Render>(
        title: &'title str,
        header: Header,
        body: Body
    ) {
        @markup::doctype()
        html["lang" = "en"] {
            head {
                meta["charset" = "UTF-8"];
                meta["name" = "viewport", "content" = "width=device-width, initial-scale=1, user-scalable=1"];
                title { @title }
                @header
            }
            body {
                @body
                footer {
                    p {
                        a[href="https://github.com/friedemannsommer/email-view-tracker", target="_blank", rel="noopener noreferrer"] { "Source code" }
                    }
                }
            }
        }
    }
    Stylesheet<'path>(path: &'path str) {
        link[rel="stylesheet", href={path}, fetchpriority="high"];
    }
    PrefetchSource<'path, 'source_type>(path: &'path str, source_type: &'source_type str) {
        link[href={path}, "as"={source_type}, rel="preload", crossorigin="anonymous"];
    }
    Button<'label>(label: &'label str, button_type: ButtonType, theme: ThemeColor) {
        button["type"={button_type}, class={theme}] { @label }
    }
    ButtonLink<'url, 'label, 'class_name>(url: &'url str, label: &'label str, button_type: ButtonType, theme: ThemeColor, class_name: Option<&'class_name str>) {
        a[href={url}, class={class_name}] {
            @Button{ button_type: *button_type, label, theme: *theme }
        }
    }
}

impl<'label> Default for Button<'label> {
    fn default() -> Self {
        Self {
            button_type: ButtonType::Button,
            label: "",
            theme: ThemeColor::Dark,
        }
    }
}

impl<'url, 'label, 'class_name> Default for ButtonLink<'url, 'label, 'class_name> {
    fn default() -> Self {
        Self {
            button_type: ButtonType::Button,
            class_name: None,
            label: "",
            theme: ThemeColor::Dark,
            url: "",
        }
    }
}

impl Default for ThemeColor {
    fn default() -> Self {
        Self::Dark
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Button
    }
}

impl markup::Render for ThemeColor {
    fn render(&self, writer: &mut impl Write) -> std::fmt::Result {
        match self {
            Self::Danger => writer.write_str("danger"),
            Self::Dark => writer.write_str("dark"),
            Self::Primary => writer.write_str("primary"),
            Self::Success => writer.write_str("success"),
        }
    }
}

impl markup::Render for ButtonType {
    fn render(&self, writer: &mut impl Write) -> std::fmt::Result {
        match self {
            Self::Button => writer.write_str("button"),
            Self::Submit => writer.write_str("submit"),
        }
    }
}

impl markup::RenderAttributeValue for ButtonType {}

impl markup::RenderAttributeValue for ThemeColor {}
