use anyhow::Result;

use crate::Client;

pub struct Access {
    pub client: Client,
}

impl Access {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Access { client }
    }

    /**
     * Retrieves a list of access scopes associated to the access token.
     *
     * This function performs a `GET` to the `/admin/oauth/access_scopes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/accessscope#index-2020-10
     */
    pub async fn get_admin_oauth_scope(&self) -> Result<()> {
        let url = "/admin/oauth/access_scopes.json".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Retrieves a list of storefront access tokens that have been issued.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#index-2020-01
     */
    pub async fn deprecated_202001_get_storefront_token(&self) -> Result<()> {
        let url = "/admin/api/2020-01/storefront_access_tokens.json".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Creates a new storefront access token.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#create-2020-01
     */
    pub async fn deprecated_202001_create_storefront_tokens(
        &self,
        body: &serde_json::Value,
    ) -> Result<()> {
        let url = "/admin/api/2020-01/storefront_access_tokens.json".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Deletes an existing storefront access token.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/storefront_access_tokens/{storefront_access_token_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `storefront_access_token_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_storefront_tokens_param_token(
        &self,
        storefront_access_token_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/admin/api/2020-01/storefront_access_tokens/{}/json",
            crate::progenitor_support::encode_path(&storefront_access_token_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Retrieves a list of storefront access tokens that have been issued.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#index-2020-04
     */
    pub async fn deprecated_202004_get_storefront_token(&self) -> Result<()> {
        let url = "/admin/api/2020-04/storefront_access_tokens.json".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Creates a new storefront access token.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#create-2020-04
     */
    pub async fn deprecated_202004_create_storefront_tokens(
        &self,
        body: &serde_json::Value,
    ) -> Result<()> {
        let url = "/admin/api/2020-04/storefront_access_tokens.json".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Deletes an existing storefront access token.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/storefront_access_tokens/{storefront_access_token_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `storefront_access_token_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_storefront_tokens_param_token(
        &self,
        storefront_access_token_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/admin/api/2020-04/storefront_access_tokens/{}/json",
            crate::progenitor_support::encode_path(&storefront_access_token_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Retrieves a list of storefront access tokens that have been issued.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#index-2020-07
     */
    pub async fn deprecated_202007_get_storefront_token(&self) -> Result<()> {
        let url = "/admin/api/2020-07/storefront_access_tokens.json".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Creates a new storefront access token.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#create-2020-07
     */
    pub async fn deprecated_202007_create_storefront_tokens(
        &self,
        body: &serde_json::Value,
    ) -> Result<()> {
        let url = "/admin/api/2020-07/storefront_access_tokens.json".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Deletes an existing storefront access token.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/storefront_access_tokens/{storefront_access_token_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `storefront_access_token_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_storefront_tokens_param_token(
        &self,
        storefront_access_token_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/admin/api/2020-07/storefront_access_tokens/{}/json",
            crate::progenitor_support::encode_path(&storefront_access_token_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Retrieves a list of storefront access tokens that have been issued.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#index-2020-10
     */
    pub async fn get_storefront_token(&self) -> Result<()> {
        let url = "/admin/api/2020-10/storefront_access_tokens.json".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Creates a new storefront access token.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#create-2020-10
     */
    pub async fn create_storefront_tokens(&self, body: &serde_json::Value) -> Result<()> {
        let url = "/admin/api/2020-10/storefront_access_tokens.json".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Deletes an existing storefront access token.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/storefront_access_tokens/{storefront_access_token_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `storefront_access_token_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_storefront_tokens_param_token(
        &self,
        storefront_access_token_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/admin/api/2020-10/storefront_access_tokens/{}/json",
            crate::progenitor_support::encode_path(&storefront_access_token_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Retrieves a list of storefront access tokens that have been issued.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#index-2021-01
     */
    pub async fn deprecated_202101_get_storefront_token(&self) -> Result<()> {
        let url = "/admin/api/2021-01/storefront_access_tokens.json".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Creates a new storefront access token.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#create-2021-01
     */
    pub async fn deprecated_202101_create_storefront_tokens(
        &self,
        body: &serde_json::Value,
    ) -> Result<()> {
        let url = "/admin/api/2021-01/storefront_access_tokens.json".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Deletes an existing storefront access token.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/storefront_access_tokens/{storefront_access_token_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `storefront_access_token_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_storefront_tokens_param_token(
        &self,
        storefront_access_token_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/admin/api/2021-01/storefront_access_tokens/{}/json",
            crate::progenitor_support::encode_path(&storefront_access_token_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Retrieves a list of storefront access tokens that have been issued.
     *
     * This function performs a `GET` to the `/admin/api/unstable/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#index-unstable
     */
    pub async fn deprecated_unstable_get_storefront_token(&self) -> Result<()> {
        let url = "/admin/api/unstable/storefront_access_tokens.json".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Creates a new storefront access token.
     *
     * This function performs a `POST` to the `/admin/api/unstable/storefront_access_tokens.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#create-unstable
     */
    pub async fn deprecated_unstable_create_storefront_tokens(
        &self,
        body: &serde_json::Value,
    ) -> Result<()> {
        let url = "/admin/api/unstable/storefront_access_tokens.json".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Deletes an existing storefront access token.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/storefront_access_tokens/{storefront_access_token_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/access/storefrontaccesstoken#destroy-unstable
     *
     * **Parameters:**
     *
     * * `storefront_access_token_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_storefront_tokens_param_token(
        &self,
        storefront_access_token_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/admin/api/unstable/storefront_access_tokens/{}/json",
            crate::progenitor_support::encode_path(&storefront_access_token_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
