use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
};

use reqwest::Method;
use serde_json::json;

use crate::{client::Client, errors::ApiError, types::*};

#[derive(Debug)]
pub struct AuthenticationRoute {
    count: Mutex<usize>,
    user: Mutex<Option<UserPrivate>>,
    client: Weak<Client>,
}

impl AuthenticationRoute {
    /**
     * Creates a new `AuthenticationRoute` with an empty Authentication list.
     * The `client` parameter is an `Arc<Client<State>>` that allows the route
     */
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            count: Mutex::new(0),
            user: Mutex::new(None),
            client: Arc::downgrade(&client),
        })
    }

    pub async fn signin(&self, username: &str, password: &str) -> Result<UserPrivate, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");

        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("password", password);

        match client
            .as_ref()
            .call_api::<UserPrivate>(
                Method::POST,
                "/auth/login",
                Some(serde_json::to_value(map).unwrap()),
                None,
            )
            .await
        {
            Ok((user, _, _)) => {
                let mut count_lock = self.count.lock().unwrap();
                *count_lock += 1;
                // Update the user in the route
                let mut user_lock = self.user.lock().unwrap();
                *user_lock = Some(user.clone());
                Ok(user)
            }
            Err(e) => match e {
                ApiError::BadRequest(err) => return Err(ApiError::InvalidCredentials(err)),
                ApiError::Unauthorized(err) => return Err(ApiError::InvalidCredentials(err)),
                ApiError::RequestError(err) => return Err(ApiError::InvalidCredentials(err)),
                _ => Err(e),
            },
        }
    }

    /**
     * Returns the current user's private profile.
     * This is a convenience method that calls `me()` and returns the user data.
     * # Returns
     * - `Ok(UserPrivate)` if the user was found
     * - `Err(ApiError)` if there was an error fetching the user
     */
    pub fn get_user(&self) -> Result<UserPrivate, ApiError> {
        let ca_user = self.user.lock().unwrap();
        match &*ca_user {
            Some(user) => Ok(user.clone()),
            None => Err(ApiError::Unknown(
                "User not found. Please call `me()` to fetch the user data.".to_string(),
            )),
        }
    }

    /**
     * Fetches the authenticated user's private profile.
     * Note: This method updates the internal user state with the fetched user data.
     * # Returns
     * - `Ok(UserPrivate)` if the user was found
     * - `Err(ApiError)` if there was an error fetching the user
     */
    pub async fn me(&self) -> Result<UserPrivate, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");

        match client
            .as_ref()
            .call_api::<UserPrivate>(Method::GET, "/auth/me", None, None)
            .await
        {
            Ok((user, _, _)) => {
                // Update the count in the route
                let mut count_lock = self.count.lock().unwrap();
                *count_lock += 1;
                // Update the user in the route
                let mut user_lock = self.user.lock().unwrap();
                *user_lock = Some(user.clone());
                Ok(user)
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    /**
     * Registers a new user with the given username and password.
     * This method creates a new user account in the system.
     * # Arguments
     * - `username`: The username for the new user.
     * - `password`: The password for the new user.
     * # Returns
     * - `Ok(UserPrivate)` if the registration was successful.
     * - `Err(ApiError)` if there was an error during registration.
     */
    pub async fn register(&self, username: &str, password: &str) -> Result<UserPrivate, ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("password", password);
        match client
            .as_ref()
            .call_api::<UserPrivate>(
                Method::POST,
                "/users",
                Some(serde_json::to_value(map).unwrap()),
                None,
            )
            .await
        {
            Ok((user, _, _)) => {
                // Update the user in the route
                let mut user_lock = self.user.lock().unwrap();
                *user_lock = Some(user.clone());
                Ok(user)
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn get_count(&self) -> usize {
        let count_lock = self.count.lock().unwrap();
        *count_lock
    }
    /**
     * Creates a new `AuthenticationRoute` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(old: &AuthenticationRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            count: Mutex::new(old.count.lock().unwrap().clone()),
            user: Mutex::new(old.user.lock().unwrap().clone()), // Clone the user state
            client: Arc::downgrade(&client),
        })
    }
}
