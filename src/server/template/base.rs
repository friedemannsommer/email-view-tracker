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
            }
        }
    }
}
