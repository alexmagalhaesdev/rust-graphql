use crate::user_service::User;
use async_graphql::ID;

pub struct DB;

impl DB {
    pub fn get_data(&self) -> Vec<User> {
        vec![
            User {
                id: ID::from("1"),
                name: String::from("Alexsander"),
                email: String::from("summer.almeida@gmail.com"),
                phone: String::from("123-456-7890"),
                address: String::from("123 Street"),
                city: String::from("Fake City"),
                organization: String::from("LLC Company"),
            },
            User {
                id: ID::from("2"),
                name: String::from("Julia"),
                email: String::from("julia.julia@gmail.com"),
                phone: String::from("113-555-7890"),
                address: String::from("73 Street Manson"),
                city: String::from("All City"),
                organization: String::from("Farms"),
            },
            User {
                id: ID::from("3"),
                name: String::from("Bernardo"),
                email: String::from("bern.dos@gmail.com"),
                phone: String::from("123-250-0000"),
                address: String::from("953 Street"),
                city: String::from("Another City"),
                organization: String::from("LLC Dist"),
            },
            User {
                id: ID::from("4"),
                name: String::from("Lucas"),
                email: String::from("lucas.zang@gmail.com"),
                phone: String::from("123-789-9999"),
                address: String::from("123 Avenue"),
                city: String::from("Some City"),
                organization: String::from("LLC Tech Inc."),
            },
        ]
    }
}
