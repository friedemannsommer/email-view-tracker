#![allow(clippy::needless_lifetimes)] // the templates require a lifetime by the generator macro

markup::define! {
    Header<'a>(
        title: &'a str,
        user: &'a entity::user::ActiveModel
    ) {
        header {
            h1 { @title }
            p {
                "Hi "
                a[href="/profile", title="Go to profile"] {
                    @user.name.as_ref()
                }
                ", "
                a[href="/logout", title="Logout"] {
                    "logout"
                }
                "?"
            }
        }
    }
}
