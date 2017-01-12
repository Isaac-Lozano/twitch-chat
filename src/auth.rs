pub struct Auth
{
    pub username: String,
    pub oauth: String,
}

impl Auth
{
    pub fn new<T>(username: T, oauth: T) -> Auth
        where T: Into<String>
    {
        Auth
        {
            username: username.into(),
            oauth: oauth.into(),
        }
    }
}
