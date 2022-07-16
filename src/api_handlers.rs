use serde::Deserialize;

const BASE_API_URL: &str = "https://random-data-ap&&&i.com/api";

#[derive(Deserialize, Debug, Default)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

impl User {
    #[allow(dead_code)]
    fn new(id: u32, first_name: &str, middle_name: Option<&str>, last_name: &str) -> Self {
        let middle_name_converted = if let Some(mname) = middle_name {
            Some(mname.to_string())
        } else {
            None
        };

        Self {
            id,
            first_name: first_name.to_string(),
            middle_name: middle_name_converted,
            last_name: last_name.to_string(),
        }
    }

    pub fn get_full_name(self: &Self) -> String {
        let middle_name_unwrapped: String = if let Some(mname) = &self.middle_name {
            mname.clone()
        } else {
            String::from("")
        };

        format!(
            "{} {} {}",
            &(self.first_name),
            &middle_name_unwrapped,
            &(self.last_name)
        )
        .to_string()
    }
}

#[derive(Deserialize, Default)]
pub struct UsersResponse {
    pub users: Vec<User>,
}

impl UsersResponse {
    fn new() -> Self {
        Self { users: Vec::new() }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UsersError {
    #[error("Failed fetching users' details")]
    FetchFailedErr(#[from] reqwest::Error),
}

// generates dummy users
pub async fn fetch_users() -> Result<UsersResponse, UsersError> {
    let mut users_response = UsersResponse::new();

    for n in 1..21 {
        // fetch 20 users
        let user: User = User::new(
            n,
            &format!("First #{}", n),
            Some(&format!("Middle #{}", n)),
            &format!("Last #{}", n),
        );

        users_response.users.push(user);
    }

    Ok(users_response)
}

// fetches users from actual api -- To be used when we have all the layout and styling solved
pub async fn fetch_actual_users() -> Result<UsersResponse, UsersError> {
    let request_url = format!(
        "{api_url}/{path}",
        api_url = BASE_API_URL,
        path = "name/random_name"
    );

    println!("Request URL: {}", request_url);

    let mut users_response = UsersResponse::new();

    for _ in 1..21 {
        // fetch 20 users
        let response = reqwest::get(&request_url).await?;

        let user: User = response.json().await?;

        users_response.users.push(user);
    }

    Ok(users_response)
}
