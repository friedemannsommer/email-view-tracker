use super::base::{PrefetchSource, Stylesheet};

#[derive(Debug, Copy, Clone)]
pub enum StylesheetVariant {
    Login,
    User,
}

pub fn get_default_header(stylesheet: StylesheetVariant) -> impl markup::Render {
    markup::new! {
        @PrefetchSource{ path: "/fonts/montserrat/regular.woff2", source_type: "font" }
        @PrefetchSource{ path: "/fonts/montserrat/bold.woff2", source_type: "font" }
        @Stylesheet{
            path: match stylesheet{
                StylesheetVariant::Login => "/css/login.css",
                StylesheetVariant::User => "/css/user.css"
            }
        }
    }
}
