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
        html {
            head {
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
    ButtonLink<'url, 'label>(url: &'url str, label: &'label str, button_type: ButtonType, theme: ThemeColor) {
        a[href={url}] {
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

impl<'url, 'label> Default for ButtonLink<'url, 'label> {
    fn default() -> Self {
        Self {
            button_type: ButtonType::Button,
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
