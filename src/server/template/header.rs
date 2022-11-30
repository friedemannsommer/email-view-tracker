markup::define! {
    Header<'title, 'user>(
        title: &'title str,
        user: &'user entity::user::ActiveModel
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
