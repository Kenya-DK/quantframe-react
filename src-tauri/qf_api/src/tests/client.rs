use crate::{Client, errors::ApiError};

#[tokio::test]
async fn print_token() {
    let user = "";
    let pass = "";

    assert!(!user.is_empty());
    assert!(!pass.is_empty());

    let mut client = Client::new(
        "N/A",
        "default",
        "v1",
        "https://example.com",
        true,
        "https://example.com",
        "https://example.com",
        "https://example.com",
        "https://example.com",
        "https://example.com",
        false,
    );
    match client.authentication().signin(&user, &pass).await {
        Ok(_) => {
            // client.print_info();
        }
        Err(e) => match e {
            ApiError::InvalidCredentials(err) => {
                println!("Invalid credentials: {}", err);
            }
            _ => {
                println!("Error signing in: {}", e);
            }
        },
    }
    client.set_token("new_token");
}
