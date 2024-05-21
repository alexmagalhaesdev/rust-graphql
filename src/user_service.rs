#[derive(clone)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub organization: String,
}

#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn id(&self) -> &str {
        &self.id
    }

    async fn id(&self) -> &str {
        &self.id
    }

    async fn id(&self) -> &str {
        &self.id
    }

    async fn id(&self) -> &str {
        &self.id
    }
}
