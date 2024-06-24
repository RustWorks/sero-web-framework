#[cfg(test)]
pub mod tests {
    use crate::{api::tests::delete, Details};
    use axum::http::{HeaderName, HeaderValue, StatusCode};
    use axum_test::TestServer as TestClient;
    use std::fmt::Display;

    pub async fn teardown<S, T>(client: &TestClient, subdomain: S, token: T) -> Result<(), (StatusCode, Details)>
    where
        S: AsRef<str>,
        T: Display,
    {
        let response = delete(client, "/api/site", Option::<()>::None)
            .add_header(
                HeaderName::from_static("x-subdomain"),
                HeaderValue::from_str(subdomain.as_ref()).expect("Failed to convert subdomain name to header value!"),
            )
            .authorization_bearer(token)
            .await;
        match response.status_code().is_success() {
            true => Ok(()),
            false => Err((response.status_code(), response.json())),
        }
    }
}
