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
                @Stylesheet { path: "/css/shared.css" }
                @header
            }
            body {
                @body
            }
        }
    }
    Stylesheet<'path>(path: &'path str) {
        link[rel="stylesheet", href={path}, fetchpriority="high"];
    }
}
