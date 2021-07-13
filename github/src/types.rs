//! The data types sent to and returned from the API client.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Simple User
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct User {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
     * Simple User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    /**
     * Simple User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(default)]
    pub site_admin: bool,
    /**
     * Simple User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The set of permissions for the GitHub app
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Permissions {
    /**
     * The set of permissions for the GitHub app
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub checks: String,
    /**
     * The set of permissions for the GitHub app
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents: String,
    /**
     * The set of permissions for the GitHub app
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments: String,
    /**
     * The set of permissions for the GitHub app
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues: String,
    /**
     * The set of permissions for the GitHub app
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,
}

/// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Integration {
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The list of events for the GitHub app
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the GitHub app
     */
    #[serde(default)]
    pub id: i64,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installations_count: Option<i64>,
    /**
     * The name of the GitHub app
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Simple User
     */
    #[serde()]
    pub owner: User,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pem: String,
    /**
     * The set of permissions for the GitHub app
     */
    #[serde()]
    pub permissions: Permissions,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub webhook_secret: String,
}

/// Basic Error
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BasicError {
    /**
     * Basic Error
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    /**
     * Basic Error
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * Basic Error
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    /**
     * Basic Error
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Validation Error Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ValidationErrorSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    /**
     * Validation Error Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// Configuration object of the webhook
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WebhookConfig {
    /**
     * Configuration object of the webhook
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Configuration object of the webhook
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    /**
     * Configuration object of the webhook
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    /**
     * Configuration object of the webhook
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// An enterprise account
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Enterprise {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * An enterprise account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the enterprise
     */
    #[serde(default)]
    pub id: i64,
    /**
     * The name of the enterprise.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The slug url identifier for the enterprise.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * An enterprise account
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub website_url: String,
}

/**
 * The level of permission to grant the access token for GitHub Actions workflows, workflow runs, and artifacts. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Actions {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Actions::Read => "read",
            Actions::Write => "write",
            Actions::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Actions {
    fn default() -> Actions {
        Actions::Noop
    }
}

/**
 * The level of permission to grant the access token for repository creation, deletion, settings, teams, and collaborators creation. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Administration {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Administration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Administration::Read => "read",
            Administration::Write => "write",
            Administration::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Administration {
    fn default() -> Administration {
        Administration::Noop
    }
}

/**
 * The level of permission to grant the access token for checks on code. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Checks {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Checks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Checks::Read => "read",
            Checks::Write => "write",
            Checks::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Checks {
    fn default() -> Checks {
        Checks::Noop
    }
}

/**
 * The level of permission to grant the access token for notification of content references and creation content attachments. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContentReferences {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for ContentReferences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ContentReferences::Read => "read",
            ContentReferences::Write => "write",
            ContentReferences::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ContentReferences {
    fn default() -> ContentReferences {
        ContentReferences::Noop
    }
}

/**
 * The level of permission to grant the access token for repository contents, commits, branches, downloads, releases, and merges. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Contents {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Contents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Contents::Read => "read",
            Contents::Write => "write",
            Contents::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Contents {
    fn default() -> Contents {
        Contents::Noop
    }
}

/**
 * The level of permission to grant the access token for deployments and deployment statuses. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Deployments {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Deployments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Deployments::Read => "read",
            Deployments::Write => "write",
            Deployments::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Deployments {
    fn default() -> Deployments {
        Deployments::Noop
    }
}

/**
 * The level of permission to grant the access token for managing repository environments. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Environments {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Environments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Environments::Read => "read",
            Environments::Write => "write",
            Environments::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Environments {
    fn default() -> Environments {
        Environments::Noop
    }
}

/**
 * The level of permission to grant the access token for issues and related comments, assignees, labels, and milestones. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Issues {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Issues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Issues::Read => "read",
            Issues::Write => "write",
            Issues::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Issues {
    fn default() -> Issues {
        Issues::Noop
    }
}

/**
 * The level of permission to grant the access token to search repositories, list collaborators, and access repository metadata. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Metadata {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Metadata::Read => "read",
            Metadata::Write => "write",
            Metadata::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Metadata {
    fn default() -> Metadata {
        Metadata::Noop
    }
}

/**
 * The level of permission to grant the access token for packages published to GitHub Packages. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Packages {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Packages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Packages::Read => "read",
            Packages::Write => "write",
            Packages::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Packages {
    fn default() -> Packages {
        Packages::Noop
    }
}

/**
 * The level of permission to grant the access token to retrieve Pages statuses, configuration, and builds, as well as create new builds. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Pages {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Pages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Pages::Read => "read",
            Pages::Write => "write",
            Pages::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Pages {
    fn default() -> Pages {
        Pages::Noop
    }
}

/**
 * The level of permission to grant the access token for pull requests and related comments, assignees, labels, milestones, and merges. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullRequests {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for PullRequests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullRequests::Read => "read",
            PullRequests::Write => "write",
            PullRequests::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullRequests {
    fn default() -> PullRequests {
        PullRequests::Noop
    }
}

/**
 * The level of permission to grant the access token to manage the post-receive hooks for a repository. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryHooks {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for RepositoryHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RepositoryHooks::Read => "read",
            RepositoryHooks::Write => "write",
            RepositoryHooks::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for RepositoryHooks {
    fn default() -> RepositoryHooks {
        RepositoryHooks::Noop
    }
}

/**
 * The level of permission to grant the access token to manage repository projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryProjects {
    Admin,
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for RepositoryProjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RepositoryProjects::Admin => "admin",
            RepositoryProjects::Read => "read",
            RepositoryProjects::Write => "write",
            RepositoryProjects::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for RepositoryProjects {
    fn default() -> RepositoryProjects {
        RepositoryProjects::Noop
    }
}

/**
 * The level of permission to grant the access token to view and manage secret scanning alerts. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningAlerts {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for SecretScanningAlerts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SecretScanningAlerts::Read => "read",
            SecretScanningAlerts::Write => "write",
            SecretScanningAlerts::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SecretScanningAlerts {
    fn default() -> SecretScanningAlerts {
        SecretScanningAlerts::Noop
    }
}

/**
 * The level of permission to grant the access token to manage repository secrets. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Secrets {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Secrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Secrets::Read => "read",
            Secrets::Write => "write",
            Secrets::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Secrets {
    fn default() -> Secrets {
        Secrets::Noop
    }
}

/**
 * The level of permission to grant the access token to view and manage security events like code scanning alerts. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEvents {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for SecurityEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SecurityEvents::Read => "read",
            SecurityEvents::Write => "write",
            SecurityEvents::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SecurityEvents {
    fn default() -> SecurityEvents {
        SecurityEvents::Noop
    }
}

/**
 * The level of permission to grant the access token to manage just a single file. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SingleFile {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for SingleFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SingleFile::Read => "read",
            SingleFile::Write => "write",
            SingleFile::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SingleFile {
    fn default() -> SingleFile {
        SingleFile::Noop
    }
}

/**
 * The level of permission to grant the access token for commit statuses. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Statuses {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Statuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Statuses::Read => "read",
            Statuses::Write => "write",
            Statuses::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Statuses {
    fn default() -> Statuses {
        Statuses::Noop
    }
}

/**
 * The level of permission to grant the access token to retrieve Dependabot alerts. Can be one of: `read`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VulnerabilityAlerts {
    Read,
    Noop,
}

impl std::fmt::Display for VulnerabilityAlerts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            VulnerabilityAlerts::Read => "read",
            VulnerabilityAlerts::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for VulnerabilityAlerts {
    fn default() -> VulnerabilityAlerts {
        VulnerabilityAlerts::Noop
    }
}

/**
 * The level of permission to grant the access token to update GitHub Actions workflow files. Can be one of: `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Workflows {
    Write,
    Noop,
}

impl std::fmt::Display for Workflows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Workflows::Write => "write",
            Workflows::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Workflows {
    fn default() -> Workflows {
        Workflows::Noop
    }
}

/**
 * The level of permission to grant the access token for organization teams and members. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Members {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for Members {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Members::Read => "read",
            Members::Write => "write",
            Members::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Members {
    fn default() -> Members {
        Members::Noop
    }
}

/**
 * The level of permission to grant the access token to manage access to an organization. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationAdministration {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for OrganizationAdministration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationAdministration::Read => "read",
            OrganizationAdministration::Write => "write",
            OrganizationAdministration::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationAdministration {
    fn default() -> OrganizationAdministration {
        OrganizationAdministration::Noop
    }
}

/**
 * The level of permission to grant the access token to manage the post-receive hooks for an organization. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationHooks {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for OrganizationHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationHooks::Read => "read",
            OrganizationHooks::Write => "write",
            OrganizationHooks::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationHooks {
    fn default() -> OrganizationHooks {
        OrganizationHooks::Noop
    }
}

/**
 * The level of permission to grant the access token for viewing an organization's plan. Can be one of: `read`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationPlan {
    Read,
    Noop,
}

impl std::fmt::Display for OrganizationPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationPlan::Read => "read",
            OrganizationPlan::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationPlan {
    fn default() -> OrganizationPlan {
        OrganizationPlan::Noop
    }
}

/**
 * The level of permission to grant the access token to manage organization projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationProjects {
    Admin,
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for OrganizationProjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationProjects::Admin => "admin",
            OrganizationProjects::Read => "read",
            OrganizationProjects::Write => "write",
            OrganizationProjects::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationProjects {
    fn default() -> OrganizationProjects {
        OrganizationProjects::Noop
    }
}

/**
 * The level of permission to grant the access token for organization packages published to GitHub Packages. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationPackages {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for OrganizationPackages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationPackages::Read => "read",
            OrganizationPackages::Write => "write",
            OrganizationPackages::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationPackages {
    fn default() -> OrganizationPackages {
        OrganizationPackages::Noop
    }
}

/**
 * The level of permission to grant the access token to manage organization secrets. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationSecrets {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for OrganizationSecrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationSecrets::Read => "read",
            OrganizationSecrets::Write => "write",
            OrganizationSecrets::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationSecrets {
    fn default() -> OrganizationSecrets {
        OrganizationSecrets::Noop
    }
}

/**
 * The level of permission to grant the access token to view and manage GitHub Actions self-hosted runners available to an organization. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationSelfHostedRunners {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for OrganizationSelfHostedRunners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationSelfHostedRunners::Read => "read",
            OrganizationSelfHostedRunners::Write => "write",
            OrganizationSelfHostedRunners::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationSelfHostedRunners {
    fn default() -> OrganizationSelfHostedRunners {
        OrganizationSelfHostedRunners::Noop
    }
}

/**
 * The level of permission to grant the access token to view and manage users blocked by the organization. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationUserBlocking {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for OrganizationUserBlocking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationUserBlocking::Read => "read",
            OrganizationUserBlocking::Write => "write",
            OrganizationUserBlocking::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationUserBlocking {
    fn default() -> OrganizationUserBlocking {
        OrganizationUserBlocking::Noop
    }
}

/**
 * The level of permission to grant the access token to manage team discussions and related comments. Can be one of: `read` or `write`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamDiscussions {
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for TeamDiscussions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamDiscussions::Read => "read",
            TeamDiscussions::Write => "write",
            TeamDiscussions::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for TeamDiscussions {
    fn default() -> TeamDiscussions {
        TeamDiscussions::Noop
    }
}

/// The permissions granted to the user-to-server access token.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppPermissions {
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration: Option<Administration>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<Checks>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_references: Option<ContentReferences>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<Contents>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Deployments>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environments: Option<Environments>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Issues>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Members>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration: Option<OrganizationAdministration>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_hooks: Option<OrganizationHooks>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_packages: Option<OrganizationPackages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_plan: Option<OrganizationPlan>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<OrganizationProjects>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_secrets: Option<OrganizationSecrets>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_self_hosted_runners: Option<OrganizationSelfHostedRunners>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking: Option<OrganizationUserBlocking>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<Packages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<Pages>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<PullRequests>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks: Option<RepositoryHooks>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<RepositoryProjects>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_alerts: Option<SecretScanningAlerts>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_events: Option<SecurityEvents>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_file: Option<SingleFile>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Statuses>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_discussions: Option<TeamDiscussions>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts: Option<VulnerabilityAlerts>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Workflows>,
}

/**
 * Describe whether all repositories have been selected or there's a selection involved
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RepositorySelection {
    All,
    Selected,
    Noop,
}

impl std::fmt::Display for RepositorySelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RepositorySelection::All => "all",
            RepositorySelection::Selected => "selected",
            RepositorySelection::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for RepositorySelection {
    fn default() -> RepositorySelection {
        RepositorySelection::Noop
    }
}

/// Installation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Installation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_tokens_url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub account: User,
    #[serde(default)]
    pub app_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub app_slug: String,
    /**
     * Installation
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contact_email: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    /**
     * Installation
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * The ID of the installation.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde()]
    pub permissions: AppPermissions,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    /**
     * Describe whether all repositories have been selected or there's a selection involved
     */
    #[serde()]
    pub repository_selection: RepositorySelection,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file_name: String,
    /**
     * Installation
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub single_file_paths: Vec<String>,
    #[serde()]
    pub suspended_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub suspended_by: User,
    /**
     * The ID of the user or organization this token is being scoped to.
     */
    #[serde(default)]
    pub target_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_type: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// License Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct License {
    /**
     * License Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub spdx_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositoryPermissions {
    #[serde(default)]
    pub admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    #[serde(default)]
    pub pull: bool,
    #[serde(default)]
    pub push: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Users {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositoryTemplatePermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TemplateRepository {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_issues_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Users>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RepositoryTemplatePermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pushed_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers_count: Option<i64>,
}

/// A git repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Repository {
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    /**
     * Whether the repository is archived.
     */
    #[serde(default)]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * The default branch of the repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Returns whether or not this repository disabled.
     */
    #[serde(default)]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub forks: i64,
    #[serde(default)]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    /**
     * Whether downloads are enabled.
     */
    #[serde(default)]
    pub has_downloads: bool,
    /**
     * Whether issues are enabled.
     */
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_pages: bool,
    /**
     * Whether projects are enabled.
     */
    #[serde(default)]
    pub has_projects: bool,
    /**
     * Whether the wiki is enabled.
     */
    #[serde(default)]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the repository
     */
    #[serde(default)]
    pub id: i64,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * License Simple
     */
    #[serde()]
    pub license: License,
    /**
     * A git repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    /**
     * The name of the repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(default)]
    pub open_issues: i64,
    #[serde(default)]
    pub open_issues_count: i64,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<User>,
    /**
     * Simple User
     */
    #[serde()]
    pub owner: User,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,
    /**
     * Whether the repository is private or public.
     */
    #[serde(default)]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde()]
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(default)]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    /**
     * A git repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    /**
     * A git repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<TemplateRepository>,
    /**
     * A git repository
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * A git repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    #[serde(default)]
    pub watchers: i64,
    #[serde(default)]
    pub watchers_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstallationTokenRepositorySelection {
    All,
    Selected,
    Noop,
}

impl std::fmt::Display for InstallationTokenRepositorySelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InstallationTokenRepositorySelection::All => "all",
            InstallationTokenRepositorySelection::Selected => "selected",
            InstallationTokenRepositorySelection::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for InstallationTokenRepositorySelection {
    fn default() -> InstallationTokenRepositorySelection {
        InstallationTokenRepositorySelection::Noop
    }
}

/// Authentication token for a GitHub App installed on a user or org.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InstallationToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expires_at: String,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AppPermissions>,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_selection: Option<InstallationTokenRepositorySelection>,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file: String,
    /**
     * Authentication token for a GitHub App installed on a user or org.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub single_file_paths: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Errors {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// Validation Error
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ValidationError {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    /**
     * Validation Error
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Errors>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct App {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The authorization associated with an OAuth Access.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ApplicationGrant {
    #[serde()]
    pub app: App,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * The authorization associated with an OAuth Access.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScopedInstallation {
    /**
     * Simple User
     */
    #[serde()]
    pub account: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    /**
     * The permissions granted to the user-to-server access token.
     */
    #[serde()]
    pub permissions: AppPermissions,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    /**
     * Describe whether all repositories have been selected or there's a selection involved
     */
    #[serde()]
    pub repository_selection: RepositorySelection,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub single_file_paths: Vec<String>,
}

/// The authorization for an OAuth app, GitHub App, or a Personal Access Token.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Authorization {
    #[serde()]
    pub app: App,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hashed_token: String,
    #[serde(default)]
    pub id: i64,
    /**
     * The authorization for an OAuth app, GitHub App, or a Personal Access Token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<ScopedInstallation>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    /**
     * A list of scopes that this authorization is in.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_last_eight: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * The authorization for an OAuth app, GitHub App, or a Personal Access Token.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// Code Of Conduct
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeOfConduct {
    /**
     * Code Of Conduct
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The policy that controls the organizations in the enterprise that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnabledOrganizations {
    All,
    None,
    Selected,
    Noop,
}

impl std::fmt::Display for EnabledOrganizations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EnabledOrganizations::All => "all",
            EnabledOrganizations::None => "none",
            EnabledOrganizations::Selected => "selected",
            EnabledOrganizations::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for EnabledOrganizations {
    fn default() -> EnabledOrganizations {
        EnabledOrganizations::Noop
    }
}

/**
 * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AllowedActions {
    All,
    LocalOnly,
    Selected,
    Noop,
}

impl std::fmt::Display for AllowedActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AllowedActions::All => "all",
            AllowedActions::LocalOnly => "local_only",
            AllowedActions::Selected => "selected",
            AllowedActions::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for AllowedActions {
    fn default() -> AllowedActions {
        AllowedActions::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsEnterprisePermissions {
    /**
     * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
     */
    #[serde()]
    pub allowed_actions: AllowedActions,
    /**
     * The policy that controls the organizations in the enterprise that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
     */
    #[serde()]
    pub enabled_organizations: EnabledOrganizations,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_actions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_organizations_url: String,
}

/// Organization Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SelectedActions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github_owned_allowed: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patterns_allowed: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified_allowed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RunnerGroupsEnterprise {
    #[serde(default)]
    pub allows_public_repositories: bool,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub id: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub runners_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

/**
 * The type of label. Read-only labels are applied automatically when the runner is configured.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Custom,
    ReadOnly,
    Noop,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Type::Custom => "custom",
            Type::ReadOnly => "read-only",
            Type::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Labels {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Type>,
}

/// A self hosted runner
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Runner {
    #[serde(default)]
    pub busy: bool,
    /**
     * The id of the runner.
     */
    #[serde(default)]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Labels>,
    /**
     * The name of the runner.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * The Operating System of the runner.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub os: String,
    /**
     * The status of the runner.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

/// Runner Application
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RunnerApplication {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub architecture: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub os: String,
    /**
     * Runner Application
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha_256_checksum: String,
    /**
     * Runner Application
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_download_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Data {}

/// Authentication Token
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AuthenticationToken {
    /**
     * The time this token expires
     */
    #[serde()]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /**
     * Authentication Token
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Data>,
    /**
     * Authentication Token
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    /**
     * Authentication Token
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_selection: Option<RepositorySelection>,
    /**
     * Authentication Token
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file: String,
    /**
     * The token used for authentication
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActorLocation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub country_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AuditLogEvent {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@timestamp"
    )]
    pub timestamp: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub document_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_was: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_location: Option<ActorLocation>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blocked_user: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub business: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_was: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deploy_key_fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub emoji: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_were: Option<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub explanation: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hook_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limited_availability: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub old_user: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub openssh_public_key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub org: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_visibility: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repo: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_public: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub team: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub transport_protocol_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MinutesUsedBreakdown {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub macos: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubuntu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsBillingUsage {
    /**
     * The amount of free GitHub Actions minutes available.
     */
    #[serde(default)]
    pub included_minutes: i64,
    #[serde()]
    pub minutes_used_breakdown: MinutesUsedBreakdown,
    /**
     * The sum of the free and paid GitHub Actions minutes used.
     */
    #[serde(default)]
    pub total_minutes_used: i64,
    /**
     * The total paid GitHub Actions minutes used.
     */
    #[serde(default)]
    pub total_paid_minutes_used: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PackagesBillingUsage {
    /**
     * Free storage space (GB) for GitHub Packages.
     */
    #[serde(default)]
    pub included_gigabytes_bandwidth: i64,
    /**
     * Sum of the free and paid storage space (GB) for GitHuub Packages.
     */
    #[serde(default)]
    pub total_gigabytes_bandwidth_used: i64,
    /**
     * Total paid storage space (GB) for GitHuub Packages.
     */
    #[serde(default)]
    pub total_paid_gigabytes_bandwidth_used: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CombinedBillingUsage {
    /**
     * Numbers of days left in billing cycle.
     */
    #[serde(default)]
    pub days_left_in_billing_cycle: i64,
    /**
     * Estimated storage space (GB) used in billing cycle.
     */
    #[serde(default)]
    pub estimated_paid_storage_for_month: i64,
    /**
     * Estimated sum of free and paid storage space (GB) used in billing cycle.
     */
    #[serde(default)]
    pub estimated_storage_for_month: i64,
}

/// Actor
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Actor {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
     * Actor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Color-coded labels help you categorize and filter your issues (just like labels in Gmail).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Label {
    /**
     * 6-character hex code, without the leading #, identifying the color
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(default)]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default)]
    pub id: i64,
    /**
     * The name of the label.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * URL for the label
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The state of the milestone.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Closed,
    Open,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            State::Closed => "closed",
            State::Open => "open",
        }
        .fmt(f)
    }
}

impl Default for State {
    fn default() -> State {
        State::Open
    }
}

/// A collection of related issues and pull requests.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Milestone {
    #[serde()]
    pub closed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub closed_issues: i64,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub creator: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub due_on: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The number of the milestone.
     */
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub open_issues: i64,
    /**
     * The state of the milestone.
     */
    #[serde()]
    pub state: State,
    /**
     * The title of the milestone.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * How the author is associated with the repository.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuthorAssociation {
    Collaborator,
    Contributor,
    FirstTimer,
    FirstTimeContributor,
    Mannequin,
    Member,
    None,
    Owner,
    Noop,
}

impl std::fmt::Display for AuthorAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AuthorAssociation::Collaborator => "COLLABORATOR",
            AuthorAssociation::Contributor => "CONTRIBUTOR",
            AuthorAssociation::FirstTimer => "FIRST_TIMER",
            AuthorAssociation::FirstTimeContributor => "FIRST_TIME_CONTRIBUTOR",
            AuthorAssociation::Mannequin => "MANNEQUIN",
            AuthorAssociation::Member => "MEMBER",
            AuthorAssociation::None => "NONE",
            AuthorAssociation::Owner => "OWNER",
            AuthorAssociation::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for AuthorAssociation {
    fn default() -> AuthorAssociation {
        AuthorAssociation::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Issue Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueSimple {
    /**
     * Issue Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<User>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * Issue Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Issue Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Issue Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde()]
    pub closed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Label>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(default)]
    pub locked: bool,
    /**
     * A collection of related issues and pull requests.
     */
    #[serde()]
    pub milestone: Milestone,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default)]
    pub number: i64,
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    /**
     * Issue Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Issue Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeline_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionRollup {
    #[serde(default, rename = "+1")]
    pub plus_one: i64,
    #[serde(default, rename = "-1")]
    pub minus_one: i64,
    #[serde(default)]
    pub confused: i64,
    #[serde(default)]
    pub eyes: i64,
    #[serde(default)]
    pub heart: i64,
    #[serde(default)]
    pub hooray: i64,
    #[serde(default)]
    pub laugh: i64,
    #[serde(default)]
    pub rocket: i64,
    #[serde(default)]
    pub total_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Comments provide a way for people to collaborate on an issue.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueComment {
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * Comments provide a way for people to collaborate on an issue.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Comments provide a way for people to collaborate on an issue.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Comments provide a way for people to collaborate on an issue.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the issue comment
     */
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Comments provide a way for people to collaborate on an issue.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    /**
     * Comments provide a way for people to collaborate on an issue.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * URL for the issue comment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Repo {
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EventPayloadPages {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub page_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Payload {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<IssueComment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<IssueSimple>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<EventPayloadPages>,
}

/// Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Event {
    /**
     * Actor
     */
    #[serde()]
    pub actor: Actor,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org: Option<Actor>,
    #[serde()]
    pub payload: Payload,
    #[serde(default)]
    pub public: bool,
    #[serde()]
    pub repo: Repo,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// Hypermedia Link with Type
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LinkWithType {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub href: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Links {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user: Option<LinkWithType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_actor: Option<LinkWithType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_organization: Option<LinkWithType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_user_organizations: Vec<LinkWithType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_public: Option<LinkWithType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_advisories: Option<LinkWithType>,
    /**
     * Hypermedia Link with Type
     */
    #[serde()]
    pub timeline: LinkWithType,
    /**
     * Hypermedia Link with Type
     */
    #[serde()]
    pub user: LinkWithType,
}

/// Feed
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Feed {
    #[serde()]
    pub links: Links,
    /**
     * Feed
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_actor_url: String,
    /**
     * Feed
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_organization_url: String,
    /**
     * Feed
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_user_organization_urls: Vec<String>,
    /**
     * Feed
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_public_url: String,
    /**
     * Feed
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_url: String,
    /**
     * Feed
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub security_advisories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeline_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_url: String,
}

/// Base Gist
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BaseGist {
    #[serde(default)]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub files: Data,
    /**
     * Base Gist
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forks: Vec<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_pull_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_push_url: String,
    /**
     * Base Gist
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Base Gist
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    #[serde(default)]
    pub public: bool,
    /**
     * Base Gist
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Plan {
    #[serde(default)]
    pub collaborators: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default)]
    pub private_repos: i64,
    #[serde(default)]
    pub space: i64,
}

/// Public User
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PublicUser {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bio: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_usage: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub followers: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(default)]
    pub following: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(default)]
    pub hireable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_private_repos: Option<i64>,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_gists: Option<i64>,
    #[serde(default)]
    pub public_gists: i64,
    #[serde(default)]
    pub public_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(default)]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Public User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_private_repos: Option<i64>,
    /**
     * Public User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Stats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// Gist History
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistHistory {
    /**
     * Gist History
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_status: Option<Stats>,
    /**
     * Gist History
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Gist History
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Gist History
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /**
     * Gist History
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Forks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<PublicUser>,
}

/// Gist
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ForkOf {
    #[serde(default)]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde()]
    pub files: Data,
    /**
     * Gist
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forks: Vec<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_pull_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_push_url: String,
    /**
     * Gist
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Gist
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    #[serde(default)]
    pub public: bool,
    /**
     * Gist
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// Gist Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistSimple {
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Data>,
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fork_of: Option<ForkOf>,
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forks: Vec<Forks>,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_pull_url: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_push_url: String,
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<GistHistory>,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /**
     * Gist Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Gist Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user: String,
}

/// A comment made to a gist.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistComment {
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * The comment text.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// Gist Commit
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistCommit {
    #[serde()]
    pub change_status: Stats,
    #[serde()]
    pub committed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/// Gitignore Template
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitignoreTemplate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub source: String,
}

/// Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Issue {
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<User>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde()]
    pub closed_at: chrono::DateTime<chrono::Utc>,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<User>,
    #[serde(default)]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Labels to associate with this issue; pass one or more label names to replace the set of labels on this issue; send an empty array to clear all labels from the issue; note that the labels are silently dropped for users without push access to the repository
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(default)]
    pub locked: bool,
    /**
     * A collection of related issues and pull requests.
     */
    #[serde()]
    pub milestone: Milestone,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Number uniquely identifying the issue within its repository
     */
    #[serde(default)]
    pub number: i64,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    /**
     * State of the issue; either 'open' or 'closed'
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeline_url: String,
    /**
     * Title of the issue
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * URL for the issue
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// License
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LicenseData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default)]
    pub featured: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub implementation: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitations: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub spdx_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Marketplace Listing Plan
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplaceListingPlan {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub accounts_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bullets: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default)]
    pub has_free_trial: bool,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub monthly_price_in_cents: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default)]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub price_model: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub unit_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default)]
    pub yearly_price_in_cents: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplacePendingChange {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub effective_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_installed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplacePurchase {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billing_cycle: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub free_trial_ends_on: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_installed: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_billing_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_free_trial: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
}

/// Marketplace Purchase
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplacePurchaseData {
    /**
     * Marketplace Purchase
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    /**
     * Marketplace Purchase
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marketplace_pending_change: Option<MarketplacePendingChange>,
    #[serde()]
    pub marketplace_purchase: MarketplacePurchase,
    /**
     * Marketplace Purchase
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_billing_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SshKeyFingerprints {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha256_dsa: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha256_rsa: String,
}

/// Api Overview
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ApiOverview {
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub api: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependabot: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub git: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hooks: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub importer: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<String>,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_key_fingerprints: Option<SshKeyFingerprints>,
    #[serde(default)]
    pub verifiable_password_authentication: bool,
    /**
     * Api Overview
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub web: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MinimalRepositoryPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MinimalRepositoryLicense {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub spdx_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Minimal Repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MinimalRepository {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_of_conduct: Option<CodeOfConduct>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub fork: bool,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks: Option<i64>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_pages: Option<bool>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<MinimalRepositoryLicense>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_issues: Option<i64>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_issues_count: Option<i64>,
    #[serde()]
    pub owner: Data,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<MinimalRepositoryPermissions>,
    #[serde(default)]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Data>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Minimal Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers: Option<i64>,
    /**
     * Minimal Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Subject {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub latest_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Thread
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Thread {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_read_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde()]
    pub subject: Subject,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(default)]
    pub unread: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Thread Subscription
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ThreadSubscription {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub ignored: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    /**
     * Thread Subscription
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(default)]
    pub subscribed: bool,
    /**
     * Thread Subscription
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub thread_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationFullPlan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filled_seats: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default)]
    pub private_repos: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seats: Option<i64>,
    #[serde(default)]
    pub space: i64,
}

/// Organization Full
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationFull {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billing_email: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<i64>,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_repository_permission: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_usage: Option<i64>,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub followers: i64,
    #[serde(default)]
    pub following: i64,
    #[serde(default)]
    pub has_organization_projects: bool,
    #[serde(default)]
    pub has_repository_projects: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_allowed_repository_creation_type: String,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_pages: Option<bool>,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_pages: Option<bool>,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_pages: Option<bool>,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_repositories: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owned_private_repos: Option<i64>,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<OrganizationFullPlan>,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_gists: Option<i64>,
    #[serde(default)]
    pub public_gists: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_members_url: String,
    #[serde(default)]
    pub public_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_private_repos: Option<i64>,
    /**
     * Organization Full
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
    /**
     * Organization Full
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub two_factor_requirement_enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnabledRepositories {
    All,
    None,
    Selected,
    Noop,
}

impl std::fmt::Display for EnabledRepositories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EnabledRepositories::All => "all",
            EnabledRepositories::None => "none",
            EnabledRepositories::Selected => "selected",
            EnabledRepositories::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for EnabledRepositories {
    fn default() -> EnabledRepositories {
        EnabledRepositories::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsOrganizationPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    /**
     * The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
     */
    #[serde()]
    pub enabled_repositories: EnabledRepositories,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_actions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_repositories_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RunnerGroupsOrg {
    #[serde(default)]
    pub allows_public_repositories: bool,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub id: f64,
    #[serde(default)]
    pub inherited: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited_allows_public_repositories: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub runners_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

/**
 * Visibility of a secret
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    All,
    Private,
    Selected,
    Noop,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Visibility::All => "all",
            Visibility::Private => "private",
            Visibility::Selected => "selected",
            Visibility::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::Noop
    }
}

/// Secrets for GitHub Actions for an organization.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationActionsSecret {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * The name of the secret.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Secrets for GitHub Actions for an organization.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_repositories_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * Visibility of a secret
     */
    #[serde()]
    pub visibility: Visibility,
}

/// The public key used for setting Actions Secrets.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsPublicKey {
    /**
     * The public key used for setting Actions Secrets.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * The public key used for setting Actions Secrets.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /**
     * The Base64 encoded public key.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    /**
     * The identifier for the key.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
    /**
     * The public key used for setting Actions Secrets.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    /**
     * The public key used for setting Actions Secrets.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// An object without any properties.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EmptyObject {}

/// Credential Authorization
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CredentialAuthorization {
    /**
     * Credential Authorization
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_credential_id: Option<i64>,
    /**
     * Credential Authorization
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authorized_credential_note: String,
    /**
     * Credential Authorization
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authorized_credential_title: String,
    /**
     * Credential Authorization
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_accessed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Date when the credential was authorized for use.
     */
    #[serde()]
    pub credential_authorized_at: chrono::DateTime<chrono::Utc>,
    /**
     * Unique identifier for the credential.
     */
    #[serde(default)]
    pub credential_id: i64,
    /**
     * Human-readable description of the credential type.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub credential_type: String,
    /**
     * Credential Authorization
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    /**
     * User login that owns the underlying credential.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    /**
     * Credential Authorization
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    /**
     * Credential Authorization
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_last_eight: String,
}

/// Organization Invitation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrganizationInvitation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Organization Invitation
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub failed_at: String,
    /**
     * Organization Invitation
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub failed_reason: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub invitation_teams_url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub inviter: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub role: String,
    #[serde(default)]
    pub team_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgHookConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Org Hook
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgHook {
    #[serde(default)]
    pub active: bool,
    #[serde()]
    pub config: OrgHookConfig,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ping_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect. Can be one of: `existing_users`, `contributors_only`, `collaborators_only`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InteractionGroup {
    CollaboratorsOnly,
    ContributorsOnly,
    ExistingUsers,
    Noop,
}

impl std::fmt::Display for InteractionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InteractionGroup::CollaboratorsOnly => "collaborators_only",
            InteractionGroup::ContributorsOnly => "contributors_only",
            InteractionGroup::ExistingUsers => "existing_users",
            InteractionGroup::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for InteractionGroup {
    fn default() -> InteractionGroup {
        InteractionGroup::Noop
    }
}

/// Interaction limit settings.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InteractionLimitResponse {
    #[serde()]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /**
     * The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect. Can be one of: `existing_users`, `contributors_only`, `collaborators_only`.
     */
    #[serde()]
    pub limit: InteractionGroup,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub origin: String,
}

/**
 * The duration of the interaction restriction. Can be one of: `one_day`, `three_days`, `one_week`, `one_month`, `six_months`. Default: `one_day`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InteractionExpiry {
    OneDay,
    OneMonth,
    OneWeek,
    SixMonths,
    ThreeDays,
    Noop,
}

impl std::fmt::Display for InteractionExpiry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InteractionExpiry::OneDay => "one_day",
            InteractionExpiry::OneMonth => "one_month",
            InteractionExpiry::OneWeek => "one_week",
            InteractionExpiry::SixMonths => "six_months",
            InteractionExpiry::ThreeDays => "three_days",
            InteractionExpiry::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for InteractionExpiry {
    fn default() -> InteractionExpiry {
        InteractionExpiry::Noop
    }
}

/// Limit interactions to a specific type of user for a specified duration
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct InteractionLimit {
    /**
     * Limit interactions to a specific type of user for a specified duration
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<InteractionExpiry>,
    /**
     * The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect. Can be one of: `existing_users`, `contributors_only`, `collaborators_only`.
     */
    #[serde()]
    pub limit: InteractionGroup,
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Parent {
    /**
     * Description of the team
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the team
     */
    #[serde(default)]
    pub id: i64,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ldap_dn: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    /**
     * Name of the team
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Permission that the team will have for its repositories
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub privacy: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    /**
     * URL for the team
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamPermissions {
    #[serde(default)]
    pub admin: bool,
    #[serde(default)]
    pub maintain: bool,
    #[serde(default)]
    pub pull: bool,
    #[serde(default)]
    pub push: bool,
    #[serde(default)]
    pub triage: bool,
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Team {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde()]
    pub parent: Parent,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<TeamPermissions>,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub privacy: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrgMembershipState {
    Active,
    Pending,
    Noop,
}

impl std::fmt::Display for OrgMembershipState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrgMembershipState::Active => "active",
            OrgMembershipState::Pending => "pending",
            OrgMembershipState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrgMembershipState {
    fn default() -> OrgMembershipState {
        OrgMembershipState::Noop
    }
}

/**
 * The user's membership type in the organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    BillingManager,
    Member,
    Noop,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Role::Admin => "admin",
            Role::BillingManager => "billing_manager",
            Role::Member => "member",
            Role::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Role {
    fn default() -> Role {
        Role::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgMembershipPermissions {
    #[serde(default)]
    pub can_create_repository: bool,
}

/// Org Membership
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgMembership {
    /**
     * Organization Simple
     */
    #[serde()]
    pub organization: OrganizationSimple,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_url: String,
    /**
     * Org Membership
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<OrgMembershipPermissions>,
    /**
     * The user's membership type in the organization.
     */
    #[serde()]
    pub role: Role,
    /**
     * The state of the member in the organization. The `pending` state indicates the user has not yet accepted an invitation.
     */
    #[serde()]
    pub state: OrgMembershipState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// A migration.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Migration {
    /**
     * A migration.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * A migration.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<serde_json::Value>,
    #[serde(default)]
    pub exclude_attachments: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub guid: String,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub lock_repositories: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Simple User
     */
    #[serde()]
    pub owner: User,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PackageType {
    Container,
    Docker,
    Maven,
    Npm,
    Nuget,
    Rubygems,
    Noop,
}

impl std::fmt::Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PackageType::Container => "container",
            PackageType::Docker => "docker",
            PackageType::Maven => "maven",
            PackageType::Npm => "npm",
            PackageType::Nuget => "nuget",
            PackageType::Rubygems => "rubygems",
            PackageType::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PackageType {
    fn default() -> PackageType {
        PackageType::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PackageVisibility {
    Private,
    Public,
    Noop,
}

impl std::fmt::Display for PackageVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PackageVisibility::Private => "private",
            PackageVisibility::Public => "public",
            PackageVisibility::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PackageVisibility {
    fn default() -> PackageVisibility {
        PackageVisibility::Noop
    }
}

/// A software package
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Package {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the package.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * The name of the package.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A software package
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    #[serde()]
    pub package_type: PackageType,
    /**
     * A software package
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<MinimalRepository>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * The number of versions of the package.
     */
    #[serde(default)]
    pub version_count: i64,
    #[serde()]
    pub visibility: PackageVisibility,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Container {
    #[serde()]
    pub tags: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Docker {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PackageVersionMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker: Option<Docker>,
    #[serde()]
    pub package_type: PackageType,
}

/// A version of a software package
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PackageVersion {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * A version of a software package
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * A version of a software package
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * A version of a software package
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the package version.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * A version of a software package
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub license: String,
    /**
     * A version of a software package
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PackageVersionMetadata>,
    /**
     * The name of the package version.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub package_html_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The baseline permission that all organization members have on this project. Only present if owner is an organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationPermission {
    Admin,
    None,
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for OrganizationPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrganizationPermission::Admin => "admin",
            OrganizationPermission::None => "none",
            OrganizationPermission::Read => "read",
            OrganizationPermission::Write => "write",
            OrganizationPermission::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrganizationPermission {
    fn default() -> OrganizationPermission {
        OrganizationPermission::Noop
    }
}

/// Projects are a way to organize columns and cards of work.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Project {
    /**
     * Body of the project
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub columns_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub creator: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Name of the project
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default)]
    pub number: i64,
    /**
     * Projects are a way to organize columns and cards of work.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_permission: Option<OrganizationPermission>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub owner_url: String,
    /**
     * Projects are a way to organize columns and cards of work.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /**
     * State of the project; either 'open' or 'closed'
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Groups {
    /**
     * a description of the group
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_description: String,
    /**
     * The ID of the group
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_id: String,
    /**
     * The name of the group
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub synced_at: String,
}

/// External Groups to be mapped to a team for membership
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GroupMapping {
    /**
     * External Groups to be mapped to a team for membership
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Groups>,
}

/**
 * The level of privacy this team should have
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Privacy {
    Closed,
    Secret,
    Noop,
}

impl std::fmt::Display for Privacy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Privacy::Closed => "closed",
            Privacy::Secret => "secret",
            Privacy::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Privacy {
    fn default() -> Privacy {
        Privacy::Noop
    }
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamFull {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the team
     */
    #[serde(default)]
    pub id: i64,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ldap_dn: String,
    #[serde(default)]
    pub members_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    /**
     * Name of the team
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Organization Full
     */
    #[serde()]
    pub organization: OrganizationFull,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
    /**
     * Permission that the team will have for its repositories
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    /**
     * Groups of organization members that gives permissions on specified repositories.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
    #[serde(default)]
    pub repos_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * URL for the team
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A team discussion is a persistent record of a free-form conversation within a team.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamDiscussion {
    /**
     * Simple User
     */
    #[serde()]
    pub author: User,
    /**
     * The main text of the discussion.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_version: String,
    #[serde(default)]
    pub comments_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde()]
    pub last_edited_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The unique sequence number of a team discussion.
     */
    #[serde(default)]
    pub number: i64,
    /**
     * Whether or not this discussion should be pinned for easy retrieval.
     */
    #[serde(default)]
    pub pinned: bool,
    /**
     * Whether or not this discussion should be restricted to team members and organization administrators.
     */
    #[serde(default)]
    pub private: bool,
    /**
     * A team discussion is a persistent record of a free-form conversation within a team.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub team_url: String,
    /**
     * The title of the discussion.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A reply to a discussion within a team.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamDiscussionComment {
    /**
     * Simple User
     */
    #[serde()]
    pub author: User,
    /**
     * The main text of the comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_version: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discussion_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde()]
    pub last_edited_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The unique sequence number of a team discussion comment.
     */
    #[serde(default)]
    pub number: i64,
    /**
     * A reply to a discussion within a team.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The reaction to use
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Content {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Content::PlusOne => "+1",
            Content::MinusOne => "-1",
            Content::Confused => "confused",
            Content::Eyes => "eyes",
            Content::Heart => "heart",
            Content::Hooray => "hooray",
            Content::Laugh => "laugh",
            Content::Rocket => "rocket",
            Content::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Content {
    fn default() -> Content {
        Content::Noop
    }
}

/// Reactions to conversations provide a way to help people express their feelings more simply and effectively.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Reaction {
    /**
     * The reaction to use
     */
    #[serde()]
    pub content: Content,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/**
 * The role of the user in the team.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamMembershipRole {
    Maintainer,
    Member,
}

impl std::fmt::Display for TeamMembershipRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamMembershipRole::Maintainer => "maintainer",
            TeamMembershipRole::Member => "member",
        }
        .fmt(f)
    }
}

impl Default for TeamMembershipRole {
    fn default() -> TeamMembershipRole {
        TeamMembershipRole::Member
    }
}

/**
 * The state of the user's membership in the team.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamMembershipState {
    Active,
    Pending,
    Noop,
}

impl std::fmt::Display for TeamMembershipState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamMembershipState::Active => "active",
            TeamMembershipState::Pending => "pending",
            TeamMembershipState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for TeamMembershipState {
    fn default() -> TeamMembershipState {
        TeamMembershipState::Noop
    }
}

/// Team Membership
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamMembership {
    /**
     * The role of the user in the team.
     */
    #[serde()]
    pub role: TeamMembershipRole,
    /**
     * The state of the user's membership in the team.
     */
    #[serde()]
    pub state: TeamMembershipState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamProjectPermissions {
    #[serde(default)]
    pub admin: bool,
    #[serde(default)]
    pub read: bool,
    #[serde(default)]
    pub write: bool,
}

/// A team's access to a project.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamProject {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub columns_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * Simple User
     */
    #[serde()]
    pub creator: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default)]
    pub number: i64,
    /**
     * A team's access to a project.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_permission: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub owner_url: String,
    #[serde()]
    pub permissions: TeamProjectPermissions,
    /**
     * A team's access to a project.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A team's access to a repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamRepository {
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    /**
     * Whether the repository is archived.
     */
    #[serde(default)]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * The default branch of the repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Returns whether or not this repository disabled.
     */
    #[serde(default)]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub forks: i64,
    #[serde(default)]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    /**
     * Whether downloads are enabled.
     */
    #[serde(default)]
    pub has_downloads: bool,
    /**
     * Whether issues are enabled.
     */
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_pages: bool,
    /**
     * Whether projects are enabled.
     */
    #[serde(default)]
    pub has_projects: bool,
    /**
     * Whether the wiki is enabled.
     */
    #[serde(default)]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the repository
     */
    #[serde(default)]
    pub id: i64,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * License Simple
     */
    #[serde()]
    pub license: License,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    /**
     * The name of the repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(default)]
    pub open_issues: i64,
    #[serde(default)]
    pub open_issues_count: i64,
    /**
     * Simple User
     */
    #[serde()]
    pub owner: User,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,
    /**
     * Whether the repository is private or public.
     */
    #[serde(default)]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde()]
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(default)]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Data>,
    /**
     * A team's access to a repository.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * A team's access to a repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    #[serde(default)]
    pub watchers: i64,
    #[serde(default)]
    pub watchers_count: i64,
}

/// Project cards represent a scope of work.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectCard {
    /**
     * Project cards represent a scope of work.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /**
     * Project cards represent a scope of work.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub column_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub column_url: String,
    /**
     * Project cards represent a scope of work.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub creator: User,
    /**
     * The project card's ID
     */
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    /**
     * Project cards represent a scope of work.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Project columns contain cards of work.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectColumn {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cards_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * The unique identifier of the project column
     */
    #[serde(default)]
    pub id: i64,
    /**
     * Name of the project column
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Repository Collaborator Permission
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositoryCollaboratorPermission {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RateLimit {
    #[serde(default)]
    pub limit: i64,
    #[serde(default)]
    pub remaining: i64,
    #[serde(default)]
    pub reset: i64,
    #[serde(default)]
    pub used: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Resources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_scanning_upload: Option<RateLimit>,
    #[serde()]
    pub core: RateLimit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graphql: Option<RateLimit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_manifest: Option<RateLimit>,
    #[serde()]
    pub search: RateLimit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_import: Option<RateLimit>,
}

/// Rate Limit Overview
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RateLimitOverview {
    #[serde()]
    pub rate: RateLimit,
    #[serde()]
    pub resources: Resources,
}

/// Code of Conduct Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeOfConductSimple {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FullRepositoryPermissions {
    #[serde(default)]
    pub admin: bool,
    #[serde(default)]
    pub pull: bool,
    #[serde(default)]
    pub push: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FullRepositorySecurityAnalysisAdvancedStatus {
    Disabled,
    Enabled,
    Noop,
}

impl std::fmt::Display for FullRepositorySecurityAnalysisAdvancedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FullRepositorySecurityAnalysisAdvancedStatus::Disabled => "disabled",
            FullRepositorySecurityAnalysisAdvancedStatus::Enabled => "enabled",
            FullRepositorySecurityAnalysisAdvancedStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for FullRepositorySecurityAnalysisAdvancedStatus {
    fn default() -> FullRepositorySecurityAnalysisAdvancedStatus {
        FullRepositorySecurityAnalysisAdvancedStatus::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SecretScanning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<FullRepositorySecurityAnalysisAdvancedStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SecurityAnalysis {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advanced_security: Option<SecretScanning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning: Option<SecretScanning>,
}

/// Full Repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FullRepository {
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anonymous_access_enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(default)]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_of_conduct: Option<CodeOfConductSimple>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Returns whether or not this repository disabled.
     */
    #[serde(default)]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub forks: i64,
    #[serde(default)]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(default)]
    pub has_downloads: bool,
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_pages: bool,
    #[serde(default)]
    pub has_projects: bool,
    #[serde(default)]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * License Simple
     */
    #[serde()]
    pub license: License,
    /**
     * Full Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default)]
    pub network_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(default)]
    pub open_issues: i64,
    #[serde(default)]
    pub open_issues_count: i64,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<User>,
    /**
     * Simple User
     */
    #[serde()]
    pub owner: User,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Repository>,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(default)]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde()]
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_and_analysis: Option<SecurityAnalysis>,
    #[serde(default)]
    pub size: i64,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(default)]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(default)]
    pub subscribers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    /**
     * Full Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<Data>,
    /**
     * Full Repository
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Full Repository
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
    #[serde(default)]
    pub watchers: i64,
    #[serde(default)]
    pub watchers_count: i64,
}

/// An artifact
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Artifact {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_download_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Whether or not the artifact has expired.
     */
    #[serde(default)]
    pub expired: bool,
    #[serde()]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub id: i64,
    /**
     * The name of the artifact.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The size in bytes of the artifact.
     */
    #[serde(default)]
    pub size_in_bytes: i64,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The phase of the lifecycle that the job is currently in.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Completed,
    InProgress,
    Queued,
    Noop,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            JobStatus::Completed => "completed",
            JobStatus::InProgress => "in_progress",
            JobStatus::Queued => "queued",
            JobStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for JobStatus {
    fn default() -> JobStatus {
        JobStatus::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Steps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The outcome of the job.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conclusion: String,
    /**
     * The name of the job.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default)]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The phase of the lifecycle that the job is currently in.
     */
    #[serde()]
    pub status: JobStatus,
}

/// Information of a job execution in a workflow run
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Job {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_run_url: String,
    /**
     * The time that the job finished, in ISO 8601 format.
     */
    #[serde()]
    pub completed_at: chrono::DateTime<chrono::Utc>,
    /**
     * The outcome of the job.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conclusion: String,
    /**
     * The SHA of the commit that is being run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * The id of the job.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * The name of the job.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The id of the associated workflow run.
     */
    #[serde(default)]
    pub run_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub run_url: String,
    /**
     * The time that the job started, in ISO 8601 format.
     */
    #[serde()]
    pub started_at: chrono::DateTime<chrono::Utc>,
    /**
     * The phase of the lifecycle that the job is currently in.
     */
    #[serde()]
    pub status: JobStatus,
    /**
     * Information of a job execution in a workflow run
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<Steps>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsRepositoryPermissions {
    /**
     * The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
     */
    #[serde()]
    pub allowed_actions: AllowedActions,
    /**
     * Whether GitHub Actions is enabled on the repository.
     */
    #[serde(default)]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub selected_actions_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Head {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde()]
    pub repo: Repo,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestMinimal {
    #[serde()]
    pub base: Head,
    #[serde()]
    pub head: Head,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Author {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Simple Commit
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct HeadCommit {
    #[serde()]
    pub author: Author,
    #[serde()]
    pub committer: Author,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde()]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tree_id: String,
}

/// An invocation of a workflow
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WorkflowRun {
    /**
     * The URL to the artifacts for the workflow run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub artifacts_url: String,
    /**
     * The URL to cancel the workflow run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cancel_url: String,
    /**
     * An invocation of a workflow
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_suite_id: Option<i64>,
    /**
     * An invocation of a workflow
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_suite_node_id: String,
    /**
     * The URL to the associated check suite.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_suite_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub conclusion: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_branch: String,
    /**
     * Simple Commit
     */
    #[serde()]
    pub head_commit: HeadCommit,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub head_repository: MinimalRepository,
    /**
     * An invocation of a workflow
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_repository_id: Option<i64>,
    /**
     * The SHA of the head commit that points to the version of the worflow being run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * The ID of the workflow run.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * The URL to the jobs for the workflow run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub jobs_url: String,
    /**
     * The URL to download the logs for the workflow run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub logs_url: String,
    /**
     * An invocation of a workflow
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pull_requests: Vec<PullRequestMinimal>,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    /**
     * The URL to rerun the workflow run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rerun_url: String,
    /**
     * The auto incrementing run number for the workflow run.
     */
    #[serde(default)]
    pub run_number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * The URL to the workflow run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * The ID of the parent workflow.
     */
    #[serde(default)]
    pub workflow_id: i64,
    /**
     * The URL to the workflow.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub workflow_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnvironmentApprovalsEnvironments {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * Whether deployment to the environment(s) was approved or rejected
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentApprovalsState {
    Approved,
    Rejected,
    Noop,
}

impl std::fmt::Display for EnvironmentApprovalsState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EnvironmentApprovalsState::Approved => "approved",
            EnvironmentApprovalsState::Rejected => "rejected",
            EnvironmentApprovalsState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for EnvironmentApprovalsState {
    fn default() -> EnvironmentApprovalsState {
        EnvironmentApprovalsState::Noop
    }
}

/// An entry in the reviews log for environment deployments
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnvironmentApprovals {
    /**
     * The comment submitted with the deployment review
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comment: String,
    /**
     * The list of environments that were approved or rejected
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environments: Vec<EnvironmentApprovalsEnvironments>,
    /**
     * Whether deployment to the environment(s) was approved or rejected
     */
    #[serde()]
    pub state: EnvironmentApprovalsState,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/**
 * The type of reviewer. Must be one of: `User` or `Team`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentReviewerType {
    Team,
    User,
    Noop,
}

impl std::fmt::Display for DeploymentReviewerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DeploymentReviewerType::Team => "Team",
            DeploymentReviewerType::User => "User",
            DeploymentReviewerType::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for DeploymentReviewerType {
    fn default() -> DeploymentReviewerType {
        DeploymentReviewerType::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Environment {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Reviewers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DeploymentReviewerType>,
}

/// Details of a deployment that is waiting for protection rules to pass
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PendingDeployment {
    /**
     * Whether the currently authenticated user can approve the deployment
     */
    #[serde(default)]
    pub current_user_can_approve: bool,
    #[serde()]
    pub environment: Environment,
    /**
     * The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<Reviewers>,
    /**
     * The set duration of the wait timer
     */
    #[serde(default)]
    pub wait_timer: i64,
    /**
     * The time that the wait timer began.
     */
    #[serde()]
    pub wait_timer_started_at: chrono::DateTime<chrono::Utc>,
}

/// A request for a specific ref(branch,sha,tag) to be deployed
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Deployment {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub creator: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Name for the target deployment environment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    /**
     * Unique identifier of the deployment
     */
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * A request for a specific ref(branch,sha,tag) to be deployed
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub original_environment: String,
    #[serde()]
    pub payload: Data,
    /**
     * A request for a specific ref(branch,sha,tag) to be deployed
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    /**
     * A request for a specific ref(branch,sha,tag) to be deployed
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    /**
     * The ref to deploy. This can be a branch, tag, or sha.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * Parameter to specify a task to execute
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub task: String,
    /**
     * A request for a specific ref(branch,sha,tag) to be deployed
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Macos {
    #[serde(default)]
    pub jobs: i64,
    #[serde(default)]
    pub total_ms: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Billable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub macos: Option<Macos>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubuntu: Option<Macos>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<Macos>,
}

/// Workflow Run Usage
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WorkflowRunUsage {
    #[serde()]
    pub billable: Billable,
    /**
     * Workflow Run Usage
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_duration_ms: Option<i64>,
}

/// Set secrets for GitHub Actions.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSecret {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * The name of the secret.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowState {
    Active,
    Deleted,
    DisabledFork,
    DisabledInactivity,
    DisabledManually,
    Noop,
}

impl std::fmt::Display for WorkflowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WorkflowState::Active => "active",
            WorkflowState::Deleted => "deleted",
            WorkflowState::DisabledFork => "disabled_fork",
            WorkflowState::DisabledInactivity => "disabled_inactivity",
            WorkflowState::DisabledManually => "disabled_manually",
            WorkflowState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for WorkflowState {
    fn default() -> WorkflowState {
        WorkflowState::Noop
    }
}

/// A GitHub Actions workflow
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Workflow {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub badge_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * A GitHub Actions workflow
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde()]
    pub state: WorkflowState,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Windows {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_ms: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WorkflowUsageBillable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub macos: Option<Windows>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubuntu: Option<Windows>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<Windows>,
}

/// Workflow Usage
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct WorkflowUsage {
    #[serde()]
    pub billable: WorkflowUsageBillable,
}

/// Protected Branch Admin Enforced
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranchAdminEnforced {
    #[serde(default)]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DismissalRestrictions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<Team>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<User>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub users_url: String,
}

/// Protected Branch Pull Request Review
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranchPullRequestReview {
    #[serde(default)]
    pub dismiss_stale_reviews: bool,
    /**
     * Protected Branch Pull Request Review
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions: Option<DismissalRestrictions>,
    #[serde(default)]
    pub require_code_owner_reviews: bool,
    /**
     * Protected Branch Pull Request Review
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<i64>,
    /**
     * Protected Branch Pull Request Review
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Teams {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub parent: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permission: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub privacy: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchRestrictionPolicyAppsOwner {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_members_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchRestrictionPolicyAppsPermissions {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub metadata: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub single_file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Apps {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<BranchRestrictionPolicyAppsOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<BranchRestrictionPolicyAppsPermissions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub slug: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
}

/// Branch Restriction Policy
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchRestrictionPolicy {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apps: Vec<Apps>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub apps_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<Teams>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<Users>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub users_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RequiredStatusChecks {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contexts_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub enforcement_level: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AllowDeletions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnforceAdmins {
    #[serde(default)]
    pub enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Branch Protection
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchProtection {
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<AllowDeletions>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<AllowDeletions>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_admins: Option<ProtectedBranchAdminEnforced>,
    /**
     * Branch Protection
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Branch Protection
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub protection_url: String,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<AllowDeletions>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<AllowDeletions>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews: Option<ProtectedBranchPullRequestReview>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_signatures: Option<EnforceAdmins>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<RequiredStatusChecks>,
    /**
     * Branch Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<BranchRestrictionPolicy>,
    /**
     * Branch Protection
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Tree {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Short Branch
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ShortBranch {
    #[serde()]
    pub commit: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default)]
    pub protected: bool,
    /**
     * Short Branch
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protection: Option<BranchProtection>,
    /**
     * Short Branch
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub protection_url: String,
}

/// Metaproperties for Git author/committer information.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitUser {
    /**
     * Metaproperties for Git author/committer information.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * Metaproperties for Git author/committer information.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Metaproperties for Git author/committer information.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Verification {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payload: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signature: String,
    #[serde(default)]
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitData {
    /**
     * Metaproperties for Git author/committer information.
     */
    #[serde()]
    pub author: GitUser,
    #[serde(default)]
    pub comment_count: i64,
    /**
     * Metaproperties for Git author/committer information.
     */
    #[serde()]
    pub committer: GitUser,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde()]
    pub tree: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Parents {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Files {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blob_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

/// Commit
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitDataType {
    /**
     * Simple User
     */
    #[serde()]
    pub author: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde()]
    pub commit: CommitData,
    /**
     * Simple User
     */
    #[serde()]
    pub committer: User,
    /**
     * Commit
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<Files>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<Parents>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Commit
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchWithProtectionLinks {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

/// Branch With Protection
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchWithProtection {
    #[serde()]
    pub links: BranchWithProtectionLinks,
    #[serde()]
    pub commit: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Branch With Protection
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pattern: String,
    #[serde(default)]
    pub protected: bool,
    /**
     * Branch Protection
     */
    #[serde()]
    pub protection: BranchProtection,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub protection_url: String,
    /**
     * Branch With Protection
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<i64>,
}

/// Status Check Policy
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct StatusCheckPolicy {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contexts_url: String,
    #[serde(default)]
    pub strict: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<Team>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<User>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub users_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RequiredPullRequestReviews {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions:
        Option<ProtectedBranchRequiredPullRequestReviewsDismissalRestrictions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranchRequiredLinearHistory {
    #[serde(default)]
    pub enabled: bool,
}

/// Branch protections protect branches
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectedBranch {
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<ProtectedBranchRequiredLinearHistory>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<ProtectedBranchRequiredLinearHistory>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_admins: Option<EnforceAdmins>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<AllowDeletions>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<ProtectedBranchRequiredLinearHistory>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_pull_request_reviews: Option<RequiredPullRequestReviews>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_signatures: Option<EnforceAdmins>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_status_checks: Option<StatusCheckPolicy>,
    /**
     * Branch protections protect branches
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<BranchRestrictionPolicy>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A deployment created as the result of an Actions check run from a workflow that references an environment
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeploymentSimple {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Name for the target deployment environment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    /**
     * Unique identifier of the deployment
     */
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * A deployment created as the result of an Actions check run from a workflow that references an environment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub original_environment: String,
    /**
     * A deployment created as the result of an Actions check run from a workflow that references an environment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    /**
     * A deployment created as the result of an Actions check run from a workflow that references an environment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * Parameter to specify a task to execute
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub task: String,
    /**
     * A deployment created as the result of an Actions check run from a workflow that references an environment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The phase of the lifecycle that the check is currently in.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunStatus {
    Completed,
    InProgress,
    Queued,
    Noop,
}

impl std::fmt::Display for CheckRunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CheckRunStatus::Completed => "completed",
            CheckRunStatus::InProgress => "in_progress",
            CheckRunStatus::Queued => "queued",
            CheckRunStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for CheckRunStatus {
    fn default() -> CheckRunStatus {
        CheckRunStatus::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Conclusion {
    ActionRequired,
    Cancelled,
    Failure,
    Neutral,
    Skipped,
    Success,
    TimedOut,
    Noop,
}

impl std::fmt::Display for Conclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Conclusion::ActionRequired => "action_required",
            Conclusion::Cancelled => "cancelled",
            Conclusion::Failure => "failure",
            Conclusion::Neutral => "neutral",
            Conclusion::Skipped => "skipped",
            Conclusion::Success => "success",
            Conclusion::TimedOut => "timed_out",
            Conclusion::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Conclusion {
    fn default() -> Conclusion {
        Conclusion::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Output {
    #[serde(default)]
    pub annotations_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub annotations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckSuite {
    #[serde(default)]
    pub id: i64,
}

/// A check performed on the code of a given code change
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckRun {
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub app: Integration,
    #[serde()]
    pub check_suite: CheckSuite,
    #[serde()]
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde()]
    pub conclusion: Conclusion,
    /**
     * A check performed on the code of a given code change
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<DeploymentSimple>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub details_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    /**
     * The SHA of the commit that is being checked.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * The id of the check.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * The name of the check.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde()]
    pub output: Output,
    #[serde()]
    pub pull_requests: serde_json::Value,
    #[serde()]
    pub started_at: chrono::DateTime<chrono::Utc>,
    /**
     * The phase of the lifecycle that the check is currently in.
     */
    #[serde()]
    pub status: CheckRunStatus,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Check Annotation
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckAnnotation {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub annotation_level: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blob_href: String,
    #[serde(default)]
    pub end_column: i64,
    #[serde(default)]
    pub end_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_details: String,
    #[serde(default)]
    pub start_column: i64,
    #[serde(default)]
    pub start_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteStatus {
    Completed,
    InProgress,
    Queued,
    Noop,
}

impl std::fmt::Display for CheckSuiteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CheckSuiteStatus::Completed => "completed",
            CheckSuiteStatus::InProgress => "in_progress",
            CheckSuiteStatus::Queued => "queued",
            CheckSuiteStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for CheckSuiteStatus {
    fn default() -> CheckSuiteStatus {
        CheckSuiteStatus::Noop
    }
}

/// A suite of checks performed on the code of a given code change
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckSuiteData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub after: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub before: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub check_runs_url: String,
    #[serde()]
    pub conclusion: Conclusion,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_branch: String,
    /**
     * Simple Commit
     */
    #[serde()]
    pub head_commit: HeadCommit,
    /**
     * The SHA of the head commit that is being checked.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub latest_check_runs_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pull_requests: Vec<PullRequestMinimal>,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde()]
    pub status: CheckSuiteStatus,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AutoTriggerChecks {
    #[serde(default)]
    pub app_id: i64,
    #[serde(default)]
    pub setting: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Preferences {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auto_trigger_checks: Vec<AutoTriggerChecks>,
}

/// Check suite configuration preferences for a repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CheckSuitePreference {
    #[serde()]
    pub preferences: Preferences,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
}

/**
 * State of a code scanning alert.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeScanningAlertState {
    Closed,
    Dismissed,
    Fixed,
    Open,
    Noop,
}

impl std::fmt::Display for CodeScanningAlertState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CodeScanningAlertState::Closed => "closed",
            CodeScanningAlertState::Dismissed => "dismissed",
            CodeScanningAlertState::Fixed => "fixed",
            CodeScanningAlertState::Open => "open",
            CodeScanningAlertState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for CodeScanningAlertState {
    fn default() -> CodeScanningAlertState {
        CodeScanningAlertState::Noop
    }
}

/**
 * The severity of the alert.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    None,
    Note,
    Warning,
    Noop,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Severity::Error => "error",
            Severity::None => "none",
            Severity::Note => "note",
            Severity::Warning => "warning",
            Severity::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Severity {
    fn default() -> Severity {
        Severity::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertRuleSummary {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAnalysisTool {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub guid: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub version: String,
}

/// Describe a region within a file for the alert.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertLocation {
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i64>,
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i64>,
    /**
     * Describe a region within a file for the alert.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
}

/**
 * A classification of the file. For example to identify it as generated.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeScanningAlertClassification {
    Generated,
    Library,
    Source,
    Test,
    Noop,
}

impl std::fmt::Display for CodeScanningAlertClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CodeScanningAlertClassification::Generated => "generated",
            CodeScanningAlertClassification::Library => "library",
            CodeScanningAlertClassification::Source => "source",
            CodeScanningAlertClassification::Test => "test",
            CodeScanningAlertClassification::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for CodeScanningAlertClassification {
    fn default() -> CodeScanningAlertClassification {
        CodeScanningAlertClassification::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Message {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertInstance {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub analysis_key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<CodeScanningAlertClassification>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<CodeScanningAlertLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<CodeScanningAlertState>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertItems {
    /**
     * The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
     */
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
     */
    #[serde()]
    pub dismissed_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub dismissed_by: User,
    /**
     * **Required when the state is dismissed.** The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
     */
    #[serde()]
    pub dismissed_reason: serde_json::Value,
    /**
     * The GitHub URL of the alert resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * The REST API URL for fetching the list of instances for an alert.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instances_url: String,
    #[serde()]
    pub most_recent_instance: CodeScanningAlertInstance,
    /**
     * The security alert number.
     */
    #[serde(default)]
    pub number: i64,
    #[serde()]
    pub rule: CodeScanningAlertRuleSummary,
    /**
     * State of a code scanning alert.
     */
    #[serde()]
    pub state: CodeScanningAlertState,
    #[serde()]
    pub tool: CodeScanningAnalysisTool,
    /**
     * The REST API URL of the alert resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The security severity of the alert.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SecuritySeverityLevel {
    Critical,
    High,
    Low,
    Medium,
    Noop,
}

impl std::fmt::Display for SecuritySeverityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SecuritySeverityLevel::Critical => "critical",
            SecuritySeverityLevel::High => "high",
            SecuritySeverityLevel::Low => "low",
            SecuritySeverityLevel::Medium => "medium",
            SecuritySeverityLevel::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SecuritySeverityLevel {
    fn default() -> SecuritySeverityLevel {
        SecuritySeverityLevel::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlertRule {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub help: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_severity_level: Option<SecuritySeverityLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAlert {
    /**
     * The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
     */
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
     */
    #[serde()]
    pub dismissed_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub dismissed_by: User,
    /**
     * **Required when the state is dismissed.** The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
     */
    #[serde()]
    pub dismissed_reason: serde_json::Value,
    /**
     * The GitHub URL of the alert resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<serde_json::Value>,
    /**
     * The REST API URL for fetching the list of instances for an alert.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub instances_url: String,
    #[serde()]
    pub most_recent_instance: CodeScanningAlertInstance,
    /**
     * The security alert number.
     */
    #[serde(default)]
    pub number: i64,
    #[serde()]
    pub rule: CodeScanningAlertRule,
    /**
     * State of a code scanning alert.
     */
    #[serde()]
    pub state: CodeScanningAlertState,
    #[serde()]
    pub tool: CodeScanningAnalysisTool,
    /**
     * The REST API URL of the alert resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * Sets the state of the code scanning alert. Can be one of `open` or `dismissed`. You must provide `dismissed_reason` when you set the state to `dismissed`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CodeScanningAlertSetState {
    Dismissed,
    Open,
    Noop,
}

impl std::fmt::Display for CodeScanningAlertSetState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CodeScanningAlertSetState::Dismissed => "dismissed",
            CodeScanningAlertSetState::Open => "open",
            CodeScanningAlertSetState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for CodeScanningAlertSetState {
    fn default() -> CodeScanningAlertSetState {
        CodeScanningAlertSetState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAnalysis {
    /**
     * Identifies the configuration under which the analysis was executed. For example, in GitHub Actions this includes the workflow filename and job name.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub analysis_key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub category: String,
    /**
     * The SHA of the commit to which the analysis you are uploading relates.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_sha: String,
    /**
     * The time that the analysis was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
     */
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub deletable: bool,
    /**
     * Identifies the variable values associated with the environment in which this analysis was performed.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error: String,
    /**
     * Unique identifier for this analysis.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * The full Git reference, formatted as `refs/heads/<branch name>`,
     *  `refs/pull/<number>/merge`, or `refs/pull/<number>/head`.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    /**
     * The total number of results in the analysis.
     */
    #[serde(default)]
    pub results_count: i64,
    /**
     * The total number of rules used in the analysis.
     */
    #[serde(default)]
    pub rules_count: i64,
    /**
     * An identifier for the upload.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sarif_id: String,
    #[serde()]
    pub tool: CodeScanningAnalysisTool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tool_name: String,
    /**
     * The REST API URL of the analysis resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Warning generated when processing the analysis
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub warning: String,
}

/// Successful deletion of a code scanning analysis
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningAnalysisDeletion {
    /**
     * Next deletable analysis in chain, with last analysis deletion confirmation
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub confirm_delete_url: String,
    /**
     * Next deletable analysis in chain, without last analysis deletion confirmation
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub next_analysis_url: String,
}

/// Scim Error
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimError {
    /**
     * Scim Error
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub detail: String,
    /**
     * Scim Error
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    /**
     * Scim Error
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * Scim Error
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    /**
     * Scim Error
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scim_type: String,
    /**
     * Scim Error
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningSarifsReceipt {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * `pending` files have not yet been processed, while `complete` means all results in the SARIF have been stored.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingStatus {
    Complete,
    Pending,
    Noop,
}

impl std::fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProcessingStatus::Complete => "complete",
            ProcessingStatus::Pending => "pending",
            ProcessingStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ProcessingStatus {
    fn default() -> ProcessingStatus {
        ProcessingStatus::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningSarifsStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub analyses_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<ProcessingStatus>,
}

/// Collaborator
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Collaborator {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
     * Collaborator
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    /**
     * Collaborator
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    /**
     * Collaborator
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(default)]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The permission associated with the invitation.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryInvitationPermissions {
    Admin,
    Maintain,
    Read,
    Triage,
    Write,
    Noop,
}

impl std::fmt::Display for RepositoryInvitationPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RepositoryInvitationPermissions::Admin => "admin",
            RepositoryInvitationPermissions::Maintain => "maintain",
            RepositoryInvitationPermissions::Read => "read",
            RepositoryInvitationPermissions::Triage => "triage",
            RepositoryInvitationPermissions::Write => "write",
            RepositoryInvitationPermissions::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for RepositoryInvitationPermissions {
    fn default() -> RepositoryInvitationPermissions {
        RepositoryInvitationPermissions::Noop
    }
}

/// Repository invitations let you manage who you collaborate with.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositoryInvitation {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Repository invitations let you manage who you collaborate with.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the repository invitation.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * Simple User
     */
    #[serde()]
    pub invitee: User,
    /**
     * Simple User
     */
    #[serde()]
    pub inviter: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The permission associated with the invitation.
     */
    #[serde()]
    pub permissions: RepositoryInvitationPermissions,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    /**
     * URL for the repository invitation
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Commit Comment
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitComment {
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(default)]
    pub position: i64,
    /**
     * Commit Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// Branch Short
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BranchShort {
    #[serde()]
    pub commit: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default)]
    pub protected: bool,
}

/// Hypermedia Link
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Link {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub href: String,
}

/**
 * The merge method to use.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MergeMethod {
    Merge,
    Rebase,
    Squash,
    Noop,
}

impl std::fmt::Display for MergeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MergeMethod::Merge => "merge",
            MergeMethod::Rebase => "rebase",
            MergeMethod::Squash => "squash",
            MergeMethod::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for MergeMethod {
    fn default() -> MergeMethod {
        MergeMethod::Noop
    }
}

/// The status of auto merging a pull request.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AutoMerge {
    /**
     * Commit message for the merge commit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_message: String,
    /**
     * Title for the merge commit message.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_title: String,
    /**
     * Simple User
     */
    #[serde()]
    pub enabled_by: User,
    /**
     * The merge method to use.
     */
    #[serde()]
    pub merge_method: MergeMethod,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestSimpleLabels {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Base {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    /**
     * A git repository
     */
    #[serde()]
    pub repo: Repository,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestSimpleLinks {
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub comments: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub commits: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub html: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub issue: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub review_comment: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub review_comments: Link,
    /**
     * Hypermedia Link
     */
    #[serde(rename = "self")]
    pub self_: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub statuses: Link,
}

/// Pull Request Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestSimple {
    #[serde()]
    pub links: PullRequestSimpleLinks,
    /**
     * Pull Request Simple
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    /**
     * Pull Request Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<User>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * The status of auto merging a pull request.
     */
    #[serde()]
    pub auto_merge: AutoMerge,
    #[serde()]
    pub base: Base,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde()]
    pub closed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_url: String,
    /**
     * Pull Request Simple
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde()]
    pub head: Base,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<PullRequestSimpleLabels>,
    #[serde(default)]
    pub locked: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merge_commit_sha: String,
    #[serde()]
    pub merged_at: chrono::DateTime<chrono::Utc>,
    /**
     * A collection of related issues and pull requests.
     */
    #[serde()]
    pub milestone: Milestone,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default)]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch_url: String,
    /**
     * Pull Request Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_reviewers: Vec<User>,
    /**
     * Pull Request Simple
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_teams: Vec<Team>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub review_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub review_comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SimpleCommitStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub context: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Combined Commit Status
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CombinedCommitStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<SimpleCommitStatus>,
    #[serde(default)]
    pub total_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The status of a commit.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Status {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub context: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    /**
     * Simple User
     */
    #[serde()]
    pub creator: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub updated_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Readme {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommunityProfileFiles {
    /**
     * Code of Conduct Simple
     */
    #[serde()]
    pub code_of_conduct: CodeOfConductSimple,
    #[serde()]
    pub code_of_conduct_file: Readme,
    #[serde()]
    pub contributing: Readme,
    #[serde()]
    pub issue_template: Readme,
    /**
     * License Simple
     */
    #[serde()]
    pub license: License,
    #[serde()]
    pub pull_request_template: Readme,
    #[serde()]
    pub readme: Readme,
}

/// Community Profile
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommunityProfile {
    /**
     * Community Profile
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_reports_enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation: String,
    #[serde()]
    pub files: CommunityProfileFiles,
    #[serde(default)]
    pub health_percentage: i64,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Diff Entry
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DiffEntry {
    #[serde(default)]
    pub additions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blob_url: String,
    #[serde(default)]
    pub changes: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(default)]
    pub deletions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub filename: String,
    /**
     * Diff Entry
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch: String,
    /**
     * Diff Entry
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_filename: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CommitComparisonStatus {
    Ahead,
    Behind,
    Diverged,
    Identical,
    Noop,
}

impl std::fmt::Display for CommitComparisonStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CommitComparisonStatus::Ahead => "ahead",
            CommitComparisonStatus::Behind => "behind",
            CommitComparisonStatus::Diverged => "diverged",
            CommitComparisonStatus::Identical => "identical",
            CommitComparisonStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for CommitComparisonStatus {
    fn default() -> CommitComparisonStatus {
        CommitComparisonStatus::Noop
    }
}

/// Commit Comparison
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitComparison {
    #[serde(default)]
    pub ahead_by: i64,
    #[serde()]
    pub base_commit: Tree,
    #[serde(default)]
    pub behind_by: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commits: Vec<Tree>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_url: String,
    /**
     * Commit Comparison
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<DiffEntry>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde()]
    pub merge_base_commit: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permalink_url: String,
    #[serde()]
    pub status: CommitComparisonStatus,
    #[serde(default)]
    pub total_commits: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Content Reference attachments allow you to provide context around URLs posted in comments
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentReferenceAttachment {
    /**
     * The body of the attachment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * The ID of the attachment
     */
    #[serde(default)]
    pub id: i64,
    /**
     * Content Reference attachments allow you to provide context around URLs posted in comments
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The title of the attachment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentTreeEntriesLinks {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Entries {
    #[serde()]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Content Tree
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentTree {
    #[serde()]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    /**
     * Content Tree
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Entries>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Content File
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentFile {
    #[serde()]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encoding: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    /**
     * Content File
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub submodule_git_url: String,
    /**
     * Content File
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// An object describing a symlink
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentSymlink {
    #[serde()]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// An object describing a symlink
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentSubmodule {
    #[serde()]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub submodule_git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitContentLinks {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<FileCommitContentLinks>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Committer {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitTree {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitParents {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitVerification {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub payload: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signature: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Committer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<Committer>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<FileCommitParents>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<FileCommitTree>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<FileCommitVerification>,
}

/// File Commit
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct FileCommitData {
    #[serde()]
    pub commit: FileCommit,
    #[serde()]
    pub content: FileCommitContent,
}

/// Contributor
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Contributor {
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(default)]
    pub contributions: i64,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Contributor
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    /**
     * Contributor
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * Contributor
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The state of the status.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentStatusState {
    Error,
    Failure,
    InProgress,
    Inactive,
    Pending,
    Queued,
    Success,
    Noop,
}

impl std::fmt::Display for DeploymentStatusState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DeploymentStatusState::Error => "error",
            DeploymentStatusState::Failure => "failure",
            DeploymentStatusState::InProgress => "in_progress",
            DeploymentStatusState::Inactive => "inactive",
            DeploymentStatusState::Pending => "pending",
            DeploymentStatusState::Queued => "queued",
            DeploymentStatusState::Success => "success",
            DeploymentStatusState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for DeploymentStatusState {
    fn default() -> DeploymentStatusState {
        DeploymentStatusState::Noop
    }
}

/// The status of a deployment.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeploymentStatus {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub creator: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployment_url: String,
    /**
     * A short description of the status.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The status of a deployment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    /**
     * The status of a deployment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * The status of a deployment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub log_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The status of a deployment.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    /**
     * The state of the status.
     */
    #[serde()]
    pub state: DeploymentStatusState,
    /**
     * Deprecated: the URL to associate with this status.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeploymentBranchPolicy {
    /**
     * Whether only branches that match the specified name patterns can deploy to this environment.  If `custom_branch_policies` is `true`, `protected_branches` must be `false`; if `custom_branch_policies` is `false`, `protected_branches` must be `true`.
     */
    #[serde(default)]
    pub custom_branch_policies: bool,
    /**
     * Whether only branches with branch protection rules can deploy to this environment. If `protected_branches` is `true`, `custom_branch_policies` must be `false`; if `protected_branches` is `false`, `custom_branch_policies` must be `true`.
     */
    #[serde(default)]
    pub protected_branches: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProtectionRules {
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_timer: Option<i64>,
}

/// Details of a deployment environment
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnvironmentData {
    /**
     * The time that the environment was created, in ISO 8601 format.
     */
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Details of a deployment environment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicy>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * The id of the environment.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * The name of the environment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Details of a deployment environment
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protection_rules: Vec<ProtectionRules>,
    /**
     * The time that the environment was last updated, in ISO 8601 format.
     */
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Short Blob
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ShortBlob {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Blob
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Blob {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encoding: String,
    /**
     * Blob
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub highlighted_content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Identifying information for the git-user
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCommitAuthor {
    /**
     * Timestamp of the commit
     */
    #[serde()]
    pub date: chrono::DateTime<chrono::Utc>,
    /**
     * Git email address of the user
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Name of the git user
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCommitTree {
    /**
     * SHA for the commit
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCommitParents {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * SHA for the commit
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Low-level Git commit operations within a repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCommit {
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub author: GitCommitAuthor,
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub committer: GitCommitAuthor,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Message describing the purpose of the commit
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<GitCommitParents>,
    /**
     * SHA for the commit
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde()]
    pub tree: GitCommitTree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde()]
    pub verification: Verification,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Object {
    /**
     * SHA for the reference
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Git references within a repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitRef {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde()]
    pub object: Object,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Tagger {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitTagObject {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Metadata for a Git tag
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitTag {
    /**
     * Message describing the purpose of the tag
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde()]
    pub object: GitTagObject,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Name of the tag
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag: String,
    #[serde()]
    pub tagger: Tagger,
    /**
     * URL for the tag
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Metadata for a Git tag
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitTree {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mode: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// The hierarchy between files in a Git repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitTreeData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Objects specifying a tree structure
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tree: Vec<GitTree>,
    #[serde(default)]
    pub truncated: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct HookResponse {
    #[serde(default)]
    pub code: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct HookConfig {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub digest: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub password: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub room: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subdomain: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Webhooks for repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Hook {
    /**
     * Determines whether the hook is actually triggered on pushes.
     */
    #[serde(default)]
    pub active: bool,
    #[serde()]
    pub config: HookConfig,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Determines what events the hook is triggered for. Default: ['push'].
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    /**
     * Unique identifier of the webhook.
     */
    #[serde(default)]
    pub id: i64,
    #[serde()]
    pub last_response: HookResponse,
    /**
     * The name of a valid service, use 'web' for a webhook.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ping_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub test_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ImportStatus {
    Auth,
    AuthFailed,
    Choose,
    Complete,
    Detecting,
    DetectionFoundMultiple,
    DetectionFoundNothing,
    DetectionNeedsAuth,
    Error,
    Importing,
    Mapping,
    None,
    Pushing,
    Setup,
    Unknown,
    WaitingToPush,
    Noop,
}

impl std::fmt::Display for ImportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ImportStatus::Auth => "auth",
            ImportStatus::AuthFailed => "auth_failed",
            ImportStatus::Choose => "choose",
            ImportStatus::Complete => "complete",
            ImportStatus::Detecting => "detecting",
            ImportStatus::DetectionFoundMultiple => "detection_found_multiple",
            ImportStatus::DetectionFoundNothing => "detection_found_nothing",
            ImportStatus::DetectionNeedsAuth => "detection_needs_auth",
            ImportStatus::Error => "error",
            ImportStatus::Importing => "importing",
            ImportStatus::Mapping => "mapping",
            ImportStatus::None => "none",
            ImportStatus::Pushing => "pushing",
            ImportStatus::Setup => "setup",
            ImportStatus::Unknown => "unknown",
            ImportStatus::WaitingToPush => "waiting_to_push",
            ImportStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ImportStatus {
    fn default() -> ImportStatus {
        ImportStatus::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectChoices {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub human_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tfvc_project: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs: String,
}

/// A repository import from an external source.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Import {
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors_count: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authors_url: String,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_count: Option<i64>,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub error_message: String,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub failed_step: String,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_large_files: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_percent: Option<i64>,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_files_count: Option<i64>,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_files_size: Option<i64>,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub project_choices: Vec<ProjectChoices>,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_percent: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde()]
    pub status: ImportStatus,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status_text: String,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svc_root: String,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_root: String,
    /**
     * A repository import from an external source.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tfvc_project: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * A repository import from an external source.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_lfs: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs: String,
    /**
     * The URL of the originating repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_url: String,
}

/// Porter Author
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PorterAuthor {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub import_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub remote_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub remote_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Porter Large File
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PorterLargeFile {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub oid: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ref_name: String,
    #[serde(default)]
    pub size: i64,
}

/// Issue Event Label
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventLabel {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventDismissedReview {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dismissal_commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dismissal_message: String,
    #[serde(default)]
    pub review_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

/// Issue Event Milestone
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventMilestone {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// Issue Event Project Card
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventProjectCard {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub column_name: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Issue Event Project Card
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_column_name: String,
    #[serde(default)]
    pub project_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Issue Event Rename
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventRename {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to: String,
}

/// Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigner: Option<User>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_review: Option<IssueEventDismissedReview>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<IssueSimple>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<IssueEventLabel>,
    /**
     * Issue Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lock_reason: String,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<IssueEventMilestone>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<IssueEventProjectCard>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename: Option<IssueEventRename>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewer: Option<User>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_team: Option<Team>,
    /**
     * Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_requester: Option<User>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LabeledIssueEventLabel {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Labeled Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LabeledIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde()]
    pub label: LabeledIssueEventLabel,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Unlabeled Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UnlabeledIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde()]
    pub label: LabeledIssueEventLabel,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Assigned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AssignedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    /**
     * Simple User
     */
    #[serde()]
    pub assigner: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Unassigned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UnassignedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    /**
     * Simple User
     */
    #[serde()]
    pub assigner: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MilestonedIssueEventMilestone {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// Milestoned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MilestonedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde()]
    pub milestone: MilestonedIssueEventMilestone,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Demilestoned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DemilestonedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde()]
    pub milestone: MilestonedIssueEventMilestone,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Rename {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub from: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub to: String,
}

/// Renamed Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RenamedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde()]
    pub rename: Rename,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Review Requested Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewRequestedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    /**
     * Review Requested Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewer: Option<User>,
    /**
     * Review Requested Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_team: Option<Team>,
    /**
     * Simple User
     */
    #[serde()]
    pub review_requester: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Review Request Removed Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewRequestRemovedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    /**
     * Review Request Removed Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewer: Option<User>,
    /**
     * Review Request Removed Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_team: Option<Team>,
    /**
     * Simple User
     */
    #[serde()]
    pub review_requester: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DismissedReview {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dismissal_commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub dismissal_message: String,
    #[serde(default)]
    pub review_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

/// Review Dismissed Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewDismissedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde()]
    pub dismissed_review: DismissedReview,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Locked Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LockedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub lock_reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AddedProjectIssueEventCard {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub column_name: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub previous_column_name: String,
    #[serde(default)]
    pub project_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub project_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Added to Project Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AddedProjectIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    /**
     * Added to Project Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<AddedProjectIssueEventCard>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Moved Column in Project Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MovedColumnInProjectIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    /**
     * Moved Column in Project Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<AddedProjectIssueEventCard>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Removed from Project Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RemovedFromProjectIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    /**
     * Removed from Project Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<AddedProjectIssueEventCard>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Converted Note to Issue Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ConvertedNoteIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    /**
     * Converted Note to Issue Issue Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_card: Option<AddedProjectIssueEventCard>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Labeled Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueEventFor {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde()]
    pub label: LabeledIssueEventLabel,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Timeline Comment Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineCommentEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * Timeline Comment Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Timeline Comment Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Timeline Comment Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the issue comment
     */
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Timeline Comment Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    /**
     * Timeline Comment Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * URL for the issue comment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Source {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<IssueSimple>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
}

/// Timeline Cross Referenced Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineCrossReferencedEvent {
    /**
     * Timeline Cross Referenced Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<User>,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde()]
    pub source: Source,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Timeline Committed Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineCommittedEvent {
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub author: GitCommitAuthor,
    /**
     * Identifying information for the git-user
     */
    #[serde()]
    pub committer: GitCommitAuthor,
    /**
     * Timeline Committed Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Message describing the purpose of the commit
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<GitCommitParents>,
    /**
     * SHA for the commit
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde()]
    pub tree: GitCommitTree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde()]
    pub verification: Verification,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Html {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineReviewedEventLinks {
    #[serde()]
    pub html: Html,
    #[serde()]
    pub pull_request: Html,
}

/// Timeline Reviewed Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineReviewedEvent {
    #[serde()]
    pub links: TimelineReviewedEventLinks,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * The text of the review.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Timeline Reviewed Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Timeline Reviewed Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    /**
     * A commit SHA for the review.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the review
     */
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pull_request_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Timeline Reviewed Event
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestReviewCommentLinks {
    #[serde()]
    pub html: Html,
    #[serde()]
    pub pull_request: Html,
    #[serde(rename = "self")]
    pub self_: Html,
}

/**
 * The side of the first line of the range for a multi-line comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StartSide {
    Left,
    Right,
}

impl std::fmt::Display for StartSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StartSide::Left => "LEFT",
            StartSide::Right => "RIGHT",
        }
        .fmt(f)
    }
}

impl Default for StartSide {
    fn default() -> StartSide {
        StartSide::Right
    }
}

/**
 * The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Left,
    Right,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Side::Left => "LEFT",
            Side::Right => "RIGHT",
        }
        .fmt(f)
    }
}

impl Default for Side {
    fn default() -> Side {
        Side::Right
    }
}

/// Pull Request Review Comments are comments on a portion of the Pull Request's diff.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestReviewComment {
    #[serde()]
    pub links: PullRequestReviewCommentLinks,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * The text of the comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    /**
     * The SHA of the commit to which the comment applies.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * The diff of the line that the comment refers to.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_hunk: String,
    /**
     * HTML URL for the pull request review comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * The ID of the pull request review comment.
     */
    #[serde(default)]
    pub id: i64,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /**
     * The node ID of the pull request review comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * The SHA of the original commit to which the comment applies.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub original_commit_id: String,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_line: Option<i64>,
    /**
     * The index of the original line in the diff to which the comment applies.
     */
    #[serde(default)]
    pub original_position: i64,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_line: Option<i64>,
    /**
     * The relative path of the file to which the comment applies.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    /**
     * The line index in the diff to which the comment applies.
     */
    #[serde(default)]
    pub position: i64,
    /**
     * The ID of the pull request review to which the comment belongs.
     */
    #[serde(default)]
    pub pull_request_review_id: i64,
    /**
     * URL for the pull request that the review comment belongs to.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pull_request_url: String,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    /**
     * Pull Request Review Comments are comments on a portion of the Pull Request's diff.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_side: Option<StartSide>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * URL for the pull request review comment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// Timeline Line Commented Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineLineCommentedEvent {
    /**
     * Timeline Line Commented Event
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<PullRequestReviewComment>,
    /**
     * Timeline Line Commented Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    /**
     * Timeline Line Commented Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
}

/// Timeline Commit Commented Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineCommitCommentedEvent {
    /**
     * Timeline Commit Commented Event
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<CommitComment>,
    /**
     * Timeline Commit Commented Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    /**
     * Timeline Commit Commented Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    /**
     * Timeline Commit Commented Event
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
}

/// Timeline Assigned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineAssignedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Timeline Unassigned Issue Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineUnassignedIssueEvent {
    /**
     * Simple User
     */
    #[serde()]
    pub actor: User,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
     */
    #[serde()]
    pub performed_via_github_app: Integration,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Timeline Event
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TimelineIssueEvents {}

/// An SSH key granting access to a single repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeployKey {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(default)]
    pub read_only: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default)]
    pub verified: bool,
}

/// Language
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Language {}

/// License Content
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LicenseContent {
    #[serde()]
    pub links: ContentTreeEntriesLinks,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encoding: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * License Simple
     */
    #[serde()]
    pub license: License,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PagesSourceHash {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PagesHttpsCertificateState {
    Approved,
    AuthorizationCreated,
    AuthorizationPending,
    AuthorizationRevoked,
    Authorized,
    BadAuthz,
    DestroyPending,
    DnsChanged,
    Errored,
    Issued,
    New,
    Uploaded,
    Noop,
}

impl std::fmt::Display for PagesHttpsCertificateState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PagesHttpsCertificateState::Approved => "approved",
            PagesHttpsCertificateState::AuthorizationCreated => "authorization_created",
            PagesHttpsCertificateState::AuthorizationPending => "authorization_pending",
            PagesHttpsCertificateState::AuthorizationRevoked => "authorization_revoked",
            PagesHttpsCertificateState::Authorized => "authorized",
            PagesHttpsCertificateState::BadAuthz => "bad_authz",
            PagesHttpsCertificateState::DestroyPending => "destroy_pending",
            PagesHttpsCertificateState::DnsChanged => "dns_changed",
            PagesHttpsCertificateState::Errored => "errored",
            PagesHttpsCertificateState::Issued => "issued",
            PagesHttpsCertificateState::New => "new",
            PagesHttpsCertificateState::Uploaded => "uploaded",
            PagesHttpsCertificateState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PagesHttpsCertificateState {
    fn default() -> PagesHttpsCertificateState {
        PagesHttpsCertificateState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PagesHttpsCertificate {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Array of the domain set and its alternate name (if it is configured)
     */
    #[serde()]
    pub domains: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::NaiveDate>,
    #[serde()]
    pub state: PagesHttpsCertificateState,
}

/**
 * The status of the most recent build of the Page.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PageStatus {
    Building,
    Built,
    Errored,
    Noop,
}

impl std::fmt::Display for PageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PageStatus::Building => "building",
            PageStatus::Built => "built",
            PageStatus::Errored => "errored",
            PageStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PageStatus {
    fn default() -> PageStatus {
        PageStatus::Noop
    }
}

/// The configuration for GitHub Pages for a repository.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Page {
    /**
     * The Pages site's custom domain
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cname: String,
    /**
     * Whether the Page has a custom 404 page.
     */
    #[serde(default)]
    pub custom_404: bool,
    /**
     * The configuration for GitHub Pages for a repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * The configuration for GitHub Pages for a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_certificate: Option<PagesHttpsCertificate>,
    /**
     * The configuration for GitHub Pages for a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_enforced: Option<bool>,
    /**
     * Whether the GitHub Pages site is publicly visible. If set to `true`, the site is accessible to anyone on the internet. If set to `false`, the site will only be accessible to users who have at least `read` access to the repository that published the site.
     */
    #[serde(default)]
    pub public: bool,
    /**
     * The configuration for GitHub Pages for a repository.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagesSourceHash>,
    /**
     * The status of the most recent build of the Page.
     */
    #[serde()]
    pub status: PageStatus,
    /**
     * The API address for accessing this Page resource.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Error {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/// Page Build
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PageBuild {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub duration: i64,
    #[serde()]
    pub error: Error,
    /**
     * Simple User
     */
    #[serde()]
    pub pusher: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Page Build Status
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PageBuildStatus {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Domain {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub caa_error: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_resolves: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforces_https: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_cname_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_mx_records_present: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub https_error: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_a_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_apex_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cloudflare_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_fastly: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_github_user_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_pages_dot_github_dot_com: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_fastly_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_https_eligible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_non_github_pages_ip_present: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_old_ip_address: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pages_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pointed_to_github_pages_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_proxied: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_served_by_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_valid_domain: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nameservers: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responds_to_https: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_be_a_record: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AltDomain {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub caa_error: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_resolves: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforces_https: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_cname_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_mx_records_present: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub host: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub https_error: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_a_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_apex_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cloudflare_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_fastly: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_github_user_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cname_to_pages_dot_github_dot_com: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_fastly_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_https_eligible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_non_github_pages_ip_present: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_old_ip_address: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pages_domain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_pointed_to_github_pages_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_proxied: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_served_by_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_valid_domain: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub nameservers: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responds_to_https: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_be_a_record: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub uri: String,
}

/// Pages Health Check Status
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PagesHealthCheck {
    /**
     * Pages Health Check Status
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt_domain: Option<AltDomain>,
    /**
     * Pages Health Check Status
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
}

/**
 * State of this Pull Request. Either `open` or `closed`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestState {
    Closed,
    Open,
    Noop,
}

impl std::fmt::Display for PullRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullRequestState::Closed => "closed",
            PullRequestState::Open => "open",
            PullRequestState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullRequestState {
    fn default() -> PullRequestState {
        PullRequestState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestHeadRepoOwner {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(default)]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestHeadRepoLicense {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub spdx_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestHeadRepo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(default)]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default)]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub forks: i64,
    #[serde(default)]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(default)]
    pub has_downloads: bool,
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_pages: bool,
    #[serde(default)]
    pub has_projects: bool,
    #[serde(default)]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    #[serde()]
    pub license: PullRequestHeadRepoLicense,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(default)]
    pub open_issues: i64,
    #[serde(default)]
    pub open_issues_count: i64,
    #[serde()]
    pub owner: PullRequestHeadRepoOwner,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(default)]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde()]
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(default)]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default)]
    pub watchers: i64,
    #[serde(default)]
    pub watchers_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestHead {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde()]
    pub repo: PullRequestHeadRepo,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde()]
    pub user: PullRequestHeadRepoOwner,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestBaseRepo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(default)]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default)]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub forks: i64,
    #[serde(default)]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(default)]
    pub has_downloads: bool,
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_pages: bool,
    #[serde(default)]
    pub has_projects: bool,
    #[serde(default)]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * License Simple
     */
    #[serde()]
    pub license: License,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(default)]
    pub open_issues: i64,
    #[serde(default)]
    pub open_issues_count: i64,
    #[serde()]
    pub owner: PullRequestHeadRepoOwner,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(default)]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde()]
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(default)]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default)]
    pub watchers: i64,
    #[serde(default)]
    pub watchers_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestBase {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde()]
    pub repo: PullRequestBaseRepo,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde()]
    pub user: PullRequestHeadRepoOwner,
}

/// Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestData {
    #[serde()]
    pub links: PullRequestSimpleLinks,
    /**
     * Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    #[serde(default)]
    pub additions: i64,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    /**
     * Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<User>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * The status of auto merging a pull request.
     */
    #[serde()]
    pub auto_merge: AutoMerge,
    #[serde()]
    pub base: PullRequestBase,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default)]
    pub changed_files: i64,
    #[serde()]
    pub closed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(default)]
    pub commits: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub deletions: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_url: String,
    /**
     * Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde()]
    pub head: PullRequestHead,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<PullRequestSimpleLabels>,
    #[serde(default)]
    pub locked: bool,
    /**
     * Indicates whether maintainers can modify the pull request.
     */
    #[serde(default)]
    pub maintainer_can_modify: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merge_commit_sha: String,
    #[serde(default)]
    pub mergeable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mergeable_state: String,
    #[serde(default)]
    pub merged: bool,
    #[serde()]
    pub merged_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub merged_by: User,
    /**
     * A collection of related issues and pull requests.
     */
    #[serde()]
    pub milestone: Milestone,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Number uniquely identifying the pull request within its repository.
     */
    #[serde(default)]
    pub number: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub patch_url: String,
    /**
     * Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    /**
     * Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_reviewers: Vec<User>,
    /**
     * Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_teams: Vec<Parent>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub review_comment_url: String,
    #[serde(default)]
    pub review_comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub review_comments_url: String,
    /**
     * State of this Pull Request. Either `open` or `closed`.
     */
    #[serde()]
    pub state: PullRequestState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    /**
     * The title of the pull request.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// Pull Request Merge Result
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestMergeResult {
    #[serde(default)]
    pub merged: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

/// Pull Request Review Request
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestReview {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<Team>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<User>,
}

/// Pull Request Reviews are reviews on pull requests.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullRequestReviewData {
    #[serde()]
    pub links: TimelineReviewedEventLinks,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * The text of the review.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Pull Request Reviews are reviews on pull requests.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Pull Request Reviews are reviews on pull requests.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    /**
     * A commit SHA for the review.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Unique identifier of the review
     */
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pull_request_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Pull Request Reviews are reviews on pull requests.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewCommentLinks {
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub html: Link,
    /**
     * Hypermedia Link
     */
    #[serde()]
    pub pull_request: Link,
    /**
     * Hypermedia Link
     */
    #[serde(rename = "self")]
    pub self_: Link,
}

/// Legacy Review Comment
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReviewComment {
    #[serde()]
    pub links: ReviewCommentLinks,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Legacy Review Comment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Legacy Review Comment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub diff_hunk: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub original_commit_id: String,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_line: Option<i64>,
    #[serde(default)]
    pub original_position: i64,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_line: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(default)]
    pub position: i64,
    #[serde(default)]
    pub pull_request_review_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pull_request_url: String,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<StartSide>,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    /**
     * Legacy Review Comment
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_side: Option<StartSide>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/**
 * State of the release asset.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseAssetState {
    Open,
    Uploaded,
    Noop,
}

impl std::fmt::Display for ReleaseAssetState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReleaseAssetState::Open => "open",
            ReleaseAssetState::Uploaded => "uploaded",
            ReleaseAssetState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReleaseAssetState {
    fn default() -> ReleaseAssetState {
        ReleaseAssetState::Noop
    }
}

/// Data related to a release.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReleaseAsset {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub browser_download_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub download_count: i64,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    /**
     * The file name of the asset.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default)]
    pub size: i64,
    /**
     * State of the release asset.
     */
    #[serde()]
    pub state: ReleaseAssetState,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub uploader: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// A release.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Release {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<ReleaseAsset>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assets_url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub author: User,
    /**
     * A release.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * A release.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * A release.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * A release.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discussion_url: String,
    /**
     * true to create a draft (unpublished) release, false to create a published one.
     */
    #[serde(default)]
    pub draft: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    /**
     * Whether to identify the release as a prerelease or a full release.
     */
    #[serde(default)]
    pub prerelease: bool,
    #[serde()]
    pub published_at: chrono::DateTime<chrono::Utc>,
    /**
     * A release.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    /**
     * The name of the tag.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tarball_url: String,
    /**
     * Specifies the commitish value that determines where the Git tag is created from.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_commitish: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub upload_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zipball_url: String,
}

/**
 * Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningAlertState {
    Open,
    Resolved,
    Noop,
}

impl std::fmt::Display for SecretScanningAlertState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SecretScanningAlertState::Open => "open",
            SecretScanningAlertState::Resolved => "resolved",
            SecretScanningAlertState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SecretScanningAlertState {
    fn default() -> SecretScanningAlertState {
        SecretScanningAlertState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SecretScanningAlert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<User>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SecretScanningAlertState>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Stargazer
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Stargazer {
    #[serde()]
    pub starred_at: chrono::DateTime<chrono::Utc>,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// Commit Activity
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitActivity {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub days: Vec<i64>,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub week: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Weeks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
}

/// Contributor Activity
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContributorActivity {
    /**
     * Simple User
     */
    #[serde()]
    pub author: User,
    #[serde(default)]
    pub total: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weeks: Vec<Weeks>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ParticipationStats {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub all: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owner: Vec<i64>,
}

/// Repository invitations let you manage who you collaborate with.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepositorySubscription {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Determines if all notifications should be blocked from this repository.
     */
    #[serde(default)]
    pub ignored: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub reason: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    /**
     * Determines if notifications should be received from this repository.
     */
    #[serde(default)]
    pub subscribed: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Tag
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Tag {
    #[serde()]
    pub commit: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tarball_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub zipball_url: String,
}

/// A topic aggregates entities that are related to a subject.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Topic {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Traffic {
    #[serde(default)]
    pub count: i64,
    #[serde()]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub uniques: i64,
}

/// Clone Traffic
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CloneTraffic {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clones: Vec<Traffic>,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub uniques: i64,
}

/// Content Traffic
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ContentTraffic {
    #[serde(default)]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(default)]
    pub uniques: i64,
}

/// Referrer Traffic
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReferrerTraffic {
    #[serde(default)]
    pub count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub referrer: String,
    #[serde(default)]
    pub uniques: i64,
}

/// View Traffic
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ViewTraffic {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub uniques: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub views: Vec<Traffic>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimGroupListEnterpriseResourcesMembers {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "$ref"
    )]
    pub ref_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Meta {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub last_modified: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimGroupListEnterpriseResources {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<ScimGroupListEnterpriseResourcesMembers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimGroupListEnterprise {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ScimGroupListEnterpriseResources>,
    #[serde(default)]
    pub items_per_page: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(default)]
    pub start_index: f64,
    #[serde(default)]
    pub total_results: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimEnterpriseGroup {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<ScimGroupListEnterpriseResourcesMembers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Name {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub family_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub given_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Emails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserListEnterpriseResourcesGroups {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserListEnterpriseResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<Emails>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ScimUserListEnterpriseResourcesGroups>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserListEnterprise {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ScimUserListEnterpriseResources>,
    #[serde(default)]
    pub items_per_page: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(default)]
    pub start_index: f64,
    #[serde(default)]
    pub total_results: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimEnterpriseUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<Emails>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ScimUserListEnterpriseResourcesGroups>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserName {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub family_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub formatted: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub given_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserEmails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Op {
    Add,
    Remove,
    Replace,
    Noop,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Op::Add => "add",
            Op::Remove => "remove",
            Op::Replace => "replace",
            Op::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Op {
    fn default() -> Op {
        Op::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Operations {
    #[serde()]
    pub op: Op,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

/// SCIM /Users provisioning endpoints
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUser {
    /**
     * The active status of the User.
     */
    #[serde(default)]
    pub active: bool,
    /**
     * SCIM /Users provisioning endpoints
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * user emails
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<ScimUserEmails>,
    /**
     * The ID of the User.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    /**
     * SCIM /Users provisioning endpoints
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<serde_json::Value>,
    /**
     * Unique identifier of an external identity
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde()]
    pub meta: ScimUserMeta,
    #[serde()]
    pub name: ScimUserName,
    /**
     * SCIM /Users provisioning endpoints
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<Operations>,
    /**
     * SCIM /Users provisioning endpoints
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<i64>,
    /**
     * SCIM schema used.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    /**
     * Configured by the admin. Could be an email, login, or username
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

/// SCIM User List
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUserList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ScimUser>,
    #[serde(default)]
    pub items_per_page: i64,
    /**
     * SCIM schema used.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    #[serde(default)]
    pub start_index: i64,
    #[serde(default)]
    pub total_results: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Matches {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indices: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SearchResultTextMatches {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fragment: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matches: Vec<Matches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub object_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub object_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub property: String,
}

/// Code Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeSearchResultItem {
    /**
     * Code Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /**
     * Code Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    /**
     * Code Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Code Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_numbers: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde(default)]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Code Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitSearchResultItemAuthor {
    #[serde()]
    pub date: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitSearchResultItem {
    #[serde()]
    pub author: CommitSearchResultItemAuthor,
    #[serde(default)]
    pub comment_count: i64,
    /**
     * Metaproperties for Git author/committer information.
     */
    #[serde()]
    pub committer: GitUser,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde()]
    pub tree: Tree,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}

/// Commit Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CommitSearchResultItemData {
    /**
     * Simple User
     */
    #[serde()]
    pub author: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde()]
    pub commit: CommitSearchResultItem,
    /**
     * Metaproperties for Git author/committer information.
     */
    #[serde()]
    pub committer: GitUser,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<FileCommitParents>,
    /**
     * Minimal Repository
     */
    #[serde()]
    pub repository: MinimalRepository,
    #[serde(default)]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    /**
     * Commit Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Issue Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssueSearchResultItem {
    /**
     * Issue Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub active_lock_reason: String,
    /**
     * Simple User
     */
    #[serde()]
    pub assignee: User,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<User>,
    /**
     * How the author is associated with the repository.
     */
    #[serde()]
    pub author_association: AuthorAssociation,
    /**
     * Issue Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Issue Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_html: String,
    /**
     * Issue Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body_text: String,
    #[serde()]
    pub closed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub comments: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<PullRequestSimpleLabels>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(default)]
    pub locked: bool,
    /**
     * A collection of related issues and pull requests.
     */
    #[serde()]
    pub milestone: Milestone,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default)]
    pub number: i64,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(default)]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
    /**
     * Issue Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    /**
     * Issue Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub timeline_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Simple User
     */
    #[serde()]
    pub user: User,
}

/// Label Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct LabelSearchResultItem {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(default)]
    pub default: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(default)]
    pub score: f64,
    /**
     * Label Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Repo Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct RepoSearchResultItem {
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub archive_url: String,
    #[serde(default)]
    pub archived: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignees_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blobs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branches_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub clone_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub collaborators_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub compare_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contents_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub contributors_url: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub deployments_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Returns whether or not this repository disabled.
     */
    #[serde(default)]
    pub disabled: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub downloads_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub forks: i64,
    #[serde(default)]
    pub forks_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub forks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub full_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_commits_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_refs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub git_url: String,
    #[serde(default)]
    pub has_downloads: bool,
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_pages: bool,
    #[serde(default)]
    pub has_projects: bool,
    #[serde(default)]
    pub has_wiki: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hooks_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_comment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub labels_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub language: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub languages_url: String,
    /**
     * License Simple
     */
    #[serde()]
    pub license: License,
    /**
     * Repo Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub master_branch: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub merges_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestones_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub mirror_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(default)]
    pub open_issues: i64,
    #[serde(default)]
    pub open_issues_count: i64,
    /**
     * Simple User
     */
    #[serde()]
    pub owner: User,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FullRepositoryPermissions>,
    #[serde(default)]
    pub private: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub pulls_url: String,
    #[serde()]
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub releases_url: String,
    #[serde(default)]
    pub score: f64,
    #[serde(default)]
    pub size: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ssh_url: String,
    #[serde(default)]
    pub stargazers_count: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub stargazers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub statuses_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscribers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscription_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub svn_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tags_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub teams_url: String,
    /**
     * Repo Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub temp_clone_token: String,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    /**
     * Repo Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub trees_url: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default)]
    pub watchers: i64,
    #[serde(default)]
    pub watchers_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TopicRelation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub relation_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Related {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_relation: Option<TopicRelation>,
}

/// Topic Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TopicSearchResultItem {
    /**
     * Topic Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<Related>,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_by: String,
    #[serde(default)]
    pub curated: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(default)]
    pub featured: bool,
    /**
     * Topic Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub logo_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    /**
     * Topic Search Result Item
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<Related>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub released: String,
    /**
     * Topic Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_count: Option<i64>,
    #[serde(default)]
    pub score: f64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub short_description: String,
    /**
     * Topic Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// User Search Result Item
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UserSearchResultItem {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bio: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hireable: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    /**
     * User Search Result Item
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_gists: Option<i64>,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_repos: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(default)]
    pub score: f64,
    #[serde(default)]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * User Search Result Item
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Private User
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PrivateUser {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub avatar_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bio: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    /**
     * Private User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_plus: Option<bool>,
    #[serde(default)]
    pub collaborators: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub disk_usage: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(default)]
    pub followers: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(default)]
    pub following: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gravatar_id: String,
    #[serde(default)]
    pub hireable: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    #[serde(default)]
    pub id: i64,
    /**
     * Private User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub ldap_dn: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organizations_url: String,
    #[serde(default)]
    pub owned_private_repos: i64,
    /**
     * Private User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(default)]
    pub private_gists: i64,
    #[serde(default)]
    pub public_gists: i64,
    #[serde(default)]
    pub public_repos: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub received_events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repos_url: String,
    #[serde(default)]
    pub site_admin: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub subscriptions_url: String,
    /**
     * Private User
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub total_private_repos: i64,
    /**
     * Private User
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
    #[serde(default)]
    pub two_factor_authentication: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// Email
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Email {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default)]
    pub primary: bool,
    #[serde(default)]
    pub verified: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub visibility: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GpgKeyEmails {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Subkeys {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_certify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_encrypt_comms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_encrypt_storage: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_sign: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expires_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_key_id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subkeys: Vec<serde_json::Value>,
}

/// A unique encryption key
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GpgKey {
    #[serde(default)]
    pub can_certify: bool,
    #[serde(default)]
    pub can_encrypt_comms: bool,
    #[serde(default)]
    pub can_encrypt_storage: bool,
    #[serde(default)]
    pub can_sign: bool,
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<GpgKeyEmails>,
    #[serde()]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
    #[serde(default)]
    pub primary_key_id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subkeys: Vec<Subkeys>,
}

/// Key
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Key {
    #[serde()]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(default)]
    pub read_only: bool,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    #[serde(default)]
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarketplaceAccount {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub login: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub node_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_billing_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/// User Marketplace Purchase
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UserMarketplacePurchase {
    #[serde()]
    pub account: MarketplaceAccount,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billing_cycle: String,
    #[serde()]
    pub free_trial_ends_on: chrono::DateTime<chrono::Utc>,
    #[serde()]
    pub next_billing_date: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub on_free_trial: bool,
    /**
     * Marketplace Listing Plan
     */
    #[serde()]
    pub plan: MarketplaceListingPlan,
    #[serde(default)]
    pub unit_count: i64,
    #[serde()]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Starred Repository
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct StarredRepository {
    /**
     * A git repository
     */
    #[serde()]
    pub repo: Repository,
    #[serde()]
    pub starred_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Contexts {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub octicon: String,
}

/// Hovercard
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Hovercard {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<Contexts>,
}

/// Key Simple
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct KeySimple {
    #[serde(default)]
    pub id: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
}

/**
 * The event types to include:
 *   
 *   - `web` - returns web (non-Git) events
 *   - `git` - returns Git events
 *   - `all` - returns both web and Git events
 *   
 *   The default is `web`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Include {
    All,
    Git,
    Web,
    Noop,
}

impl std::fmt::Display for Include {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Include::All => "all",
            Include::Git => "git",
            Include::Web => "web",
            Include::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Include {
    fn default() -> Include {
        Include::Noop
    }
}

/**
 * The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
 *   
 *   The default is `desc`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    Asc,
    Desc,
    Noop,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Order::Asc => "asc",
            Order::Desc => "desc",
            Order::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Order {
    fn default() -> Order {
        Order::Noop
    }
}

/**
 * One of `asc` (ascending) or `desc` (descending).
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Asc,
    Desc,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Direction::Asc => "asc",
            Direction::Desc => "desc",
        }
        .fmt(f)
    }
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Desc
    }
}

/**
 * One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Sort {
    Created,
    Updated,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Sort::Created => "created",
            Sort::Updated => "updated",
        }
        .fmt(f)
    }
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Created
    }
}

/**
 * The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PackageTypeData {
    Container,
    Docker,
    Maven,
    Npm,
    Nuget,
    Rubygems,
    Noop,
}

impl std::fmt::Display for PackageTypeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PackageTypeData::Container => "container",
            PackageTypeData::Docker => "docker",
            PackageTypeData::Maven => "maven",
            PackageTypeData::Npm => "npm",
            PackageTypeData::Nuget => "nuget",
            PackageTypeData::Rubygems => "rubygems",
            PackageTypeData::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PackageTypeData {
    fn default() -> PackageTypeData {
        PackageTypeData::Noop
    }
}

/**
 * Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub can set a status of `waiting` or `requested`. For a list of the possible `status` and `conclusion` options, see "[Create a check run](https://docs.github.com/rest/reference/checks#create-a-check-run)."
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunStatus {
    ActionRequired,
    Cancelled,
    Completed,
    Failure,
    InProgress,
    Neutral,
    Queued,
    Requested,
    Skipped,
    Stale,
    Success,
    TimedOut,
    Waiting,
    Noop,
}

impl std::fmt::Display for WorkflowRunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WorkflowRunStatus::ActionRequired => "action_required",
            WorkflowRunStatus::Cancelled => "cancelled",
            WorkflowRunStatus::Completed => "completed",
            WorkflowRunStatus::Failure => "failure",
            WorkflowRunStatus::InProgress => "in_progress",
            WorkflowRunStatus::Neutral => "neutral",
            WorkflowRunStatus::Queued => "queued",
            WorkflowRunStatus::Requested => "requested",
            WorkflowRunStatus::Skipped => "skipped",
            WorkflowRunStatus::Stale => "stale",
            WorkflowRunStatus::Success => "success",
            WorkflowRunStatus::TimedOut => "timed_out",
            WorkflowRunStatus::Waiting => "waiting",
            WorkflowRunStatus::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for WorkflowRunStatus {
    fn default() -> WorkflowRunStatus {
        WorkflowRunStatus::Noop
    }
}

/**
 * Returns check runs with the specified `status`. Can be one of `queued`, `in_progress`, or `completed`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StatusData {
    Completed,
    InProgress,
    Queued,
    Noop,
}

impl std::fmt::Display for StatusData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StatusData::Completed => "completed",
            StatusData::InProgress => "in_progress",
            StatusData::Queued => "queued",
            StatusData::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for StatusData {
    fn default() -> StatusData {
        StatusData::Noop
    }
}

/**
 * Must be one of: `day`, `week`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Per {
    Day,
    Week,
}

impl std::fmt::Display for Per {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Per::Day => "day",
            Per::Week => "week",
        }
        .fmt(f)
    }
}

impl Default for Per {
    fn default() -> Per {
        Per::Day
    }
}

/**
 * Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrderData {
    Asc,
    Desc,
}

impl std::fmt::Display for OrderData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrderData::Asc => "asc",
            OrderData::Desc => "desc",
        }
        .fmt(f)
    }
}

impl Default for OrderData {
    fn default() -> OrderData {
        OrderData::Desc
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetMetaRootResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub authorizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_authorizations_html_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub current_user_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub emails_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub emojis_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub events_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub feeds_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub followers_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub following_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub hub_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issue_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub issues_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub keys_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub notifications_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_teams_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub public_gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub rate_limit_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_gists_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub starred_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub topic_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_organizations_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_repositories_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_search_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsUpdateWebhookConfigAppRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsCreateInstallationAccessTokenRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AppPermissions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repository_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsDeleteAuthorizationRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsCheckTokenRequest {
    /**
     * The access_token of the OAuth application.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsDeleteTokenRequest {
    /**
     * The OAuth access token used to authenticate to the GitHub API.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsScopeTokenRequest {
    /**
     * The OAuth access token used to authenticate to the GitHub API.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AppPermissions>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repository_ids: Vec<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OauthAuthorizationsCreateAuthorizationRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OauthAuthorizationsGetCreateAuthorizationAppRequest {
    /**
     * The OAuth app client secret for which to create the token.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OauthAuthorizationsGetCreateAuthorizationAppFingerprintRequest {
    /**
     * The OAuth app client secret for which to create the token.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub client_secret: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OauthAuthorizationsUpdateAuthorizationRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_scopes: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub fingerprint: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminSetGithubActionsPermissionsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    /**
     * The policy that controls the organizations in the enterprise that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
     */
    #[serde()]
    pub enabled_organizations: EnabledOrganizations,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetEnterpriseAdminListOrgAccessSelfHostedRunnerGroupInResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<OrganizationSimple>,
    #[serde(default)]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminSetSelectedOrganizationsEnabledGithubActionsRequest {
    /**
     * List of organization IDs to enable for GitHub Actions.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_organization_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetEnterpriseAdminListSelfHostedRunnerGroupsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runner_groups: Vec<RunnerGroupsEnterprise>,
    #[serde(default)]
    pub total_count: f64,
}

/**
 * Visibility of a runner group. You can select all organizations or select individual organization. Can be one of: `all` or `selected`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility {
    All,
    Selected,
    Noop,
}

impl std::fmt::Display for EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility::All => "all",
            EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility::Selected => "selected",
            EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility {
    fn default() -> EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility {
        EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminCreateSelfHostedRunnerGroupRequest {
    /**
     * Name of the runner group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_organization_ids: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<EnterpriseAdminCreateSelfHostedRunnerGroupRequestVisibility>,
}

/**
 * Visibility of a runner group. You can select all organizations or select individual organizations. Can be one of: `all` or `selected`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseAdminUpdateSelfHostedRunnerGroupRequestVisibility {
    All,
    Selected,
}

impl std::fmt::Display for EnterpriseAdminUpdateSelfHostedRunnerGroupRequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EnterpriseAdminUpdateSelfHostedRunnerGroupRequestVisibility::All => "all",
            EnterpriseAdminUpdateSelfHostedRunnerGroupRequestVisibility::Selected => "selected",
        }
        .fmt(f)
    }
}

impl Default for EnterpriseAdminUpdateSelfHostedRunnerGroupRequestVisibility {
    fn default() -> EnterpriseAdminUpdateSelfHostedRunnerGroupRequestVisibility {
        EnterpriseAdminUpdateSelfHostedRunnerGroupRequestVisibility::All
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminUpdateSelfHostedRunnerGroupRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<EnterpriseAdminUpdateSelfHostedRunnerGroupRequestVisibility>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminSetOrgAccessSelfHostedRunnerGroupInRequest {
    /**
     * List of organization IDs that can access the runner group.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_organization_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListSelfHostedRunnersInGroupOrgResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<Runner>,
    #[serde(default)]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetSelfHostedRunnersInGroupOrgRequest {
    /**
     * List of runner IDs to add to the runner group.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetEnterpriseAdminListSelfHostedRunnersResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<Runner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
}

/// Names and content for the files that make up the gist
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistsCreateRequestFiles {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistsCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Names and content for the files that make up the gist
     */
    #[serde()]
    pub files: GistsCreateRequestFiles,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}

/// Names of files to be updated
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistsUpdateRequestFiles {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistsUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<GistsUpdateRequestFiles>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GistsCreateCommentRequest {
    /**
     * The comment text.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetAppsListInstallationReposResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub repository_selection: String,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Indicates which sorts of issues to return. Can be one of:  
 *   \* `assigned`: Issues assigned to you  
 *   \* `created`: Issues created by you  
 *   \* `mentioned`: Issues mentioning you  
 *   \* `subscribed`: Issues you're subscribed to updates for  
 *   \* `all`: All issues the authenticated user can see, regardless of participation or creation
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    All,
    Assigned,
    Created,
    Mentioned,
    Repos,
    Subscribed,
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Filter::All => "all",
            Filter::Assigned => "assigned",
            Filter::Created => "created",
            Filter::Mentioned => "mentioned",
            Filter::Repos => "repos",
            Filter::Subscribed => "subscribed",
        }
        .fmt(f)
    }
}

impl Default for Filter {
    fn default() -> Filter {
        Filter::Assigned
    }
}

/**
 * Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssuesListState {
    All,
    Closed,
    Open,
}

impl std::fmt::Display for IssuesListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IssuesListState::All => "all",
            IssuesListState::Closed => "closed",
            IssuesListState::Open => "open",
        }
        .fmt(f)
    }
}

impl Default for IssuesListState {
    fn default() -> IssuesListState {
        IssuesListState::Open
    }
}

/**
 * What to sort results by. Can be either `created`, `updated`, `comments`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssuesListSort {
    Comments,
    Created,
    Updated,
}

impl std::fmt::Display for IssuesListSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IssuesListSort::Comments => "comments",
            IssuesListSort::Created => "created",
            IssuesListSort::Updated => "updated",
        }
        .fmt(f)
    }
}

impl Default for IssuesListSort {
    fn default() -> IssuesListSort {
        IssuesListSort::Created
    }
}

/**
 * The rendering mode.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Gfm,
    Markdown,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mode::Gfm => "gfm",
            Mode::Markdown => "markdown",
        }
        .fmt(f)
    }
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Markdown
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MarkdownRenderRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /**
     * The Markdown text to render in HTML.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
}

/**
 * To return the oldest accounts first, set to `asc`. Can be one of `asc` or `desc`. Ignored without the `sort` parameter.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AppsListAccountsPlanDirection {
    Asc,
    Desc,
    Noop,
}

impl std::fmt::Display for AppsListAccountsPlanDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AppsListAccountsPlanDirection::Asc => "asc",
            AppsListAccountsPlanDirection::Desc => "desc",
            AppsListAccountsPlanDirection::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for AppsListAccountsPlanDirection {
    fn default() -> AppsListAccountsPlanDirection {
        AppsListAccountsPlanDirection::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActivityMarkNotificationsAsReadRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PostReposCreateDeploymentResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActivitySetThreadSubscriptionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
}

/**
 * Default permission level members have for organization repositories:  
 *   \* `read` - can pull, but not push to or administer this repository.  
 *   \* `write` - can pull and push, but not administer this repository.  
 *   \* `admin` - can pull, push, and administer this repository.  
 *   \* `none` - no permissions granted by default.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DefaultRepositoryPermission {
    Admin,
    None,
    Read,
    Write,
}

impl std::fmt::Display for DefaultRepositoryPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DefaultRepositoryPermission::Admin => "admin",
            DefaultRepositoryPermission::None => "none",
            DefaultRepositoryPermission::Read => "read",
            DefaultRepositoryPermission::Write => "write",
        }
        .fmt(f)
    }
}

impl Default for DefaultRepositoryPermission {
    fn default() -> DefaultRepositoryPermission {
        DefaultRepositoryPermission::Read
    }
}

/**
 * Specifies which types of repositories non-admin organization members can create. Can be one of:  
 *   \* `all` - all organization members can create public and private repositories.  
 *   \* `private` - members can create private repositories. This option is only available to repositories that are part of an organization on GitHub Enterprise Cloud.  
 *   \* `none` - only admin members can create repositories.  
 *   **Note:** This parameter is deprecated and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MembersAllowedRepositoryCreationType {
    All,
    None,
    Private,
    Noop,
}

impl std::fmt::Display for MembersAllowedRepositoryCreationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MembersAllowedRepositoryCreationType::All => "all",
            MembersAllowedRepositoryCreationType::None => "none",
            MembersAllowedRepositoryCreationType::Private => "private",
            MembersAllowedRepositoryCreationType::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for MembersAllowedRepositoryCreationType {
    fn default() -> MembersAllowedRepositoryCreationType {
        MembersAllowedRepositoryCreationType::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub billing_email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_repository_permission: Option<DefaultRepositoryPermission>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_organization_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_repository_projects: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_allowed_repository_creation_type: Option<MembersAllowedRepositoryCreationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_can_create_repositories: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetGithubPermissionsOrganizationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    /**
     * The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
     */
    #[serde()]
    pub enabled_repositories: EnabledRepositories,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListSelectedRepositoriesEnabledGithubOrganizationResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<Repository>,
    #[serde(default)]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetSelectedRepositoriesEnabledGithubOrganizationRequest {
    /**
     * List of repository IDs to enable for GitHub Actions.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListSelfHostedRunnerGroupsOrgResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runner_groups: Vec<RunnerGroupsOrg>,
    #[serde(default)]
    pub total_count: f64,
}

/**
 * Visibility of a runner group. You can select all repositories, select individual repositories, or limit access to private repositories. Can be one of: `all`, `selected`, or `private`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility {
    All,
    Private,
    Selected,
}

impl std::fmt::Display for ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility::All => "all",
            ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility::Private => "private",
            ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility::Selected => "selected",
        }
        .fmt(f)
    }
}

impl Default for ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility {
    fn default() -> ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility {
        ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility::All
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateSelfHostedRunnerGroupOrgRequest {
    /**
     * Name of the runner group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ActionsCreateSelfHostedRunnerGroupOrgRequestVisibility>,
}

/**
 * Visibility of a runner group. You can select all repositories, select individual repositories, or all private repositories. Can be one of: `all`, `selected`, or `private`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility {
    All,
    Private,
    Selected,
    Noop,
}

impl std::fmt::Display for ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility::All => "all",
            ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility::Private => "private",
            ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility::Selected => "selected",
            ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility {
    fn default() -> ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility {
        ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsUpdateSelfHostedRunnerGroupOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ActionsUpdateSelfHostedRunnerGroupOrgRequestVisibility>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListRepoAccessSelfHostedRunnerGroupInOrgResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<MinimalRepository>,
    #[serde(default)]
    pub total_count: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetRepoAccessSelfHostedRunnerGroupInOrgRequest {
    /**
     * List of repository IDs that can access the runner group.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListSelfHostedRunnersOrgResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runners: Vec<Runner>,
    #[serde(default)]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListOrgSecretsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<OrganizationActionsSecret>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Configures the access that repositories have to the organization secret. Can be one of:  
 *   \- `all` - All repositories in an organization can access the secret.  
 *   \- `private` - Private repositories in an organization can access the secret.  
 *   \- `selected` - Only specific repositories can access the secret.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionsCreateUpdateOrgSecretRequestVisibility {
    All,
    Private,
    Selected,
    Noop,
}

impl std::fmt::Display for ActionsCreateUpdateOrgSecretRequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ActionsCreateUpdateOrgSecretRequestVisibility::All => "all",
            ActionsCreateUpdateOrgSecretRequestVisibility::Private => "private",
            ActionsCreateUpdateOrgSecretRequestVisibility::Selected => "selected",
            ActionsCreateUpdateOrgSecretRequestVisibility::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ActionsCreateUpdateOrgSecretRequestVisibility {
    fn default() -> ActionsCreateUpdateOrgSecretRequestVisibility {
        ActionsCreateUpdateOrgSecretRequestVisibility::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateUpdateOrgSecretRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encrypted_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<String>,
    /**
     * Configures the access that repositories have to the organization secret. Can be one of:  
     *  \- `all` - All repositories in an organization can access the secret.  
     *  \- `private` - Private repositories in an organization can access the secret.  
     *  \- `selected` - Only specific repositories can access the secret.
     */
    #[serde()]
    pub visibility: ActionsCreateUpdateOrgSecretRequestVisibility,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListSelectedReposOrgSecretResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<MinimalRepository>,
    #[serde(default)]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetSelectedReposOrgSecretRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_repository_ids: Vec<i64>,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsCreateWebhookRequestConfig {
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub password: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    /**
     * The URL to which the payloads will be delivered.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsCreateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
     */
    #[serde()]
    pub config: OrgsCreateWebhookRequestConfig,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    /**
     * Must be passed as "web".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsUpdateWebhookRequestConfig {
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    /**
     * The URL to which the payloads will be delivered.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsUpdateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<OrgsUpdateWebhookRequestConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetAppsListInstallationsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub installations: Vec<Installation>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Specify role for new member. Can be one of:  
 *   \* `admin` - Organization owners with full administrative rights to the organization and complete access to all repositories and teams.  
 *   \* `direct_member` - Non-owner organization members with ability to see other members and join teams by invitation.  
 *   \* `billing_manager` - Non-owner organization members with ability to manage the billing settings of your organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrgsCreateInvitationRequestRole {
    Admin,
    BillingManager,
    DirectMember,
}

impl std::fmt::Display for OrgsCreateInvitationRequestRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrgsCreateInvitationRequestRole::Admin => "admin",
            OrgsCreateInvitationRequestRole::BillingManager => "billing_manager",
            OrgsCreateInvitationRequestRole::DirectMember => "direct_member",
        }
        .fmt(f)
    }
}

impl Default for OrgsCreateInvitationRequestRole {
    fn default() -> OrgsCreateInvitationRequestRole {
        OrgsCreateInvitationRequestRole::DirectMember
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsCreateInvitationRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitee_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<OrgsCreateInvitationRequestRole>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_ids: Vec<i64>,
}

/**
 * Filter members returned in the list. Can be one of:  
 *   \* `2fa_disabled` - Members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled. Available for organization owners.  
 *   \* `all` - All members the authenticated user can see.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrgsListMembersFilter {
    TwoFaDisabled,
    All,
}

impl std::fmt::Display for OrgsListMembersFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrgsListMembersFilter::TwoFaDisabled => "2fa_disabled",
            OrgsListMembersFilter::All => "all",
        }
        .fmt(f)
    }
}

impl Default for OrgsListMembersFilter {
    fn default() -> OrgsListMembersFilter {
        OrgsListMembersFilter::All
    }
}

/**
 * Filter members returned by their role. Can be one of:  
 *   \* `all` - All members of the organization, regardless of role.  
 *   \* `admin` - Organization owners.  
 *   \* `member` - Non-owner organization members.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrgsListMembersRole {
    Admin,
    All,
    Member,
}

impl std::fmt::Display for OrgsListMembersRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrgsListMembersRole::Admin => "admin",
            OrgsListMembersRole::All => "all",
            OrgsListMembersRole::Member => "member",
        }
        .fmt(f)
    }
}

impl Default for OrgsListMembersRole {
    fn default() -> OrgsListMembersRole {
        OrgsListMembersRole::All
    }
}

/**
 * The role to give the user in the organization. Can be one of:  
 *   \* `admin` - The user will become an owner of the organization.  
 *   \* `member` - The user will become a non-owner member of the organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrgsSetMembershipUserRequestRole {
    Admin,
    Member,
}

impl std::fmt::Display for OrgsSetMembershipUserRequestRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrgsSetMembershipUserRequestRole::Admin => "admin",
            OrgsSetMembershipUserRequestRole::Member => "member",
        }
        .fmt(f)
    }
}

impl Default for OrgsSetMembershipUserRequestRole {
    fn default() -> OrgsSetMembershipUserRequestRole {
        OrgsSetMembershipUserRequestRole::Member
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsSetMembershipUserRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<OrgsSetMembershipUserRequestRole>,
}

/**
 * Allowed values that can be passed to the exclude param.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Exclude {
    Repositories,
    Noop,
}

impl std::fmt::Display for Exclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Exclude::Repositories => "repositories",
            Exclude::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Exclude {
    fn default() -> Exclude {
        Exclude::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrationsStartOrgRequestExclude {
    Repositories,
    Noop,
}

impl std::fmt::Display for MigrationsStartOrgRequestExclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MigrationsStartOrgRequestExclude::Repositories => "repositories",
            MigrationsStartOrgRequestExclude::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for MigrationsStartOrgRequestExclude {
    fn default() -> MigrationsStartOrgRequestExclude {
        MigrationsStartOrgRequestExclude::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsStartOrgRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<MigrationsStartOrgRequestExclude>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_attachments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_repositories: Option<bool>,
    /**
     * A list of arrays indicating which repositories should be migrated.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
}

/**
 * Filter the list of outside collaborators. Can be one of:  
 *   \* `2fa_disabled`: Outside collaborators without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled.  
 *   \* `all`: All outside collaborators.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrgsListOutsideCollaboratorsFilter {
    TwoFaDisabled,
    All,
}

impl std::fmt::Display for OrgsListOutsideCollaboratorsFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrgsListOutsideCollaboratorsFilter::TwoFaDisabled => "2fa_disabled",
            OrgsListOutsideCollaboratorsFilter::All => "all",
        }
        .fmt(f)
    }
}

impl Default for OrgsListOutsideCollaboratorsFilter {
    fn default() -> OrgsListOutsideCollaboratorsFilter {
        OrgsListOutsideCollaboratorsFilter::All
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeleteReposResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/**
 * The state of the package, either active or deleted.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PackagesGetAllPackageVersionsOwnedByOrgState {
    Active,
    Deleted,
}

impl std::fmt::Display for PackagesGetAllPackageVersionsOwnedByOrgState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PackagesGetAllPackageVersionsOwnedByOrgState::Active => "active",
            PackagesGetAllPackageVersionsOwnedByOrgState::Deleted => "deleted",
        }
        .fmt(f)
    }
}

impl Default for PackagesGetAllPackageVersionsOwnedByOrgState {
    fn default() -> PackagesGetAllPackageVersionsOwnedByOrgState {
        PackagesGetAllPackageVersionsOwnedByOrgState::Active
    }
}

/**
 * Indicates the state of the projects to return. Can be either `open`, `closed`, or `all`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectsListOrgState {
    All,
    Closed,
    Open,
}

impl std::fmt::Display for ProjectsListOrgState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProjectsListOrgState::All => "all",
            ProjectsListOrgState::Closed => "closed",
            ProjectsListOrgState::Open => "open",
        }
        .fmt(f)
    }
}

impl Default for ProjectsListOrgState {
    fn default() -> ProjectsListOrgState {
        ProjectsListOrgState::Open
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsCreateOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * The name of the project.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
 * Specifies the types of repositories you want returned. Can be one of `all`, `public`, `private`, `forks`, `sources`, `member`, `internal`. Note: For GitHub AE, can be one of `all`, `private`, `forks`, `sources`, `member`, `internal`. Default: `all`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `type` can also be `internal`. However, the `internal` value is not yet supported when a GitHub App calls this API with an installation access token.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListOrgType {
    All,
    Forks,
    Internal,
    Member,
    Private,
    Public,
    Sources,
    Noop,
}

impl std::fmt::Display for ReposListOrgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListOrgType::All => "all",
            ReposListOrgType::Forks => "forks",
            ReposListOrgType::Internal => "internal",
            ReposListOrgType::Member => "member",
            ReposListOrgType::Private => "private",
            ReposListOrgType::Public => "public",
            ReposListOrgType::Sources => "sources",
            ReposListOrgType::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposListOrgType {
    fn default() -> ReposListOrgType {
        ReposListOrgType::Noop
    }
}

/**
 * Can be one of `created`, `updated`, `pushed`, `full_name`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListOrgSort {
    Created,
    FullName,
    Pushed,
    Updated,
}

impl std::fmt::Display for ReposListOrgSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListOrgSort::Created => "created",
            ReposListOrgSort::FullName => "full_name",
            ReposListOrgSort::Pushed => "pushed",
            ReposListOrgSort::Updated => "updated",
        }
        .fmt(f)
    }
}

impl Default for ReposListOrgSort {
    fn default() -> ReposListOrgSort {
        ReposListOrgSort::Created
    }
}

/**
 * Can be one of `asc` or `desc`. Default: when using `full_name`: `asc`, otherwise `desc`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListOrgDirection {
    Asc,
    Desc,
    Noop,
}

impl std::fmt::Display for ReposListOrgDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListOrgDirection::Asc => "asc",
            ReposListOrgDirection::Desc => "desc",
            ReposListOrgDirection::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposListOrgDirection {
    fn default() -> ReposListOrgDirection {
        ReposListOrgDirection::Noop
    }
}

/**
 * Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. Note: For GitHub Enterprise Server and GitHub AE, this endpoint will only list repositories available to all users on the enterprise. For more information, see "[Creating an internal repository](https://help.github.com/en/github/creating-cloning-and-archiving-repositories/about-repository-visibility#about-internal-repositories)" in the GitHub Help documentation.  
 *   The `visibility` parameter overrides the `private` parameter when you use both parameters with the `nebula-preview` preview header.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposCreateInOrgRequestVisibility {
    Internal,
    Private,
    Public,
    Visibility,
    Noop,
}

impl std::fmt::Display for ReposCreateInOrgRequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposCreateInOrgRequestVisibility::Internal => "internal",
            ReposCreateInOrgRequestVisibility::Private => "private",
            ReposCreateInOrgRequestVisibility::Public => "public",
            ReposCreateInOrgRequestVisibility::Visibility => "visibility",
            ReposCreateInOrgRequestVisibility::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposCreateInOrgRequestVisibility {
    fn default() -> ReposCreateInOrgRequestVisibility {
        ReposCreateInOrgRequestVisibility::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateInOrgRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_init: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gitignore_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub license_template: String,
    /**
     * The name of the repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ReposCreateInOrgRequestVisibility>,
}

/**
 * The level of privacy this team should have. The options are:  
 *   **For a non-nested team:**  
 *   \* `secret` - only visible to organization owners and members of this team.  
 *   \* `closed` - visible to all members of this organization.  
 *   Default: `secret`  
 *   **For a parent or child team:**  
 *   \* `closed` - visible to all members of this organization.  
 *   Default for child team: `closed`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamsCreateRequestPrivacy {
    Closed,
    Secret,
    Noop,
}

impl std::fmt::Display for TeamsCreateRequestPrivacy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamsCreateRequestPrivacy::Closed => "closed",
            TeamsCreateRequestPrivacy::Secret => "secret",
            TeamsCreateRequestPrivacy::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for TeamsCreateRequestPrivacy {
    fn default() -> TeamsCreateRequestPrivacy {
        TeamsCreateRequestPrivacy::Noop
    }
}

/**
 * **Deprecated**. The permission that new repositories will be added to the team with when none is specified. Can be one of:  
 *   \* `pull` - team members can pull, but not push to or administer newly-added repositories.  
 *   \* `push` - team members can pull and push, but not administer newly-added repositories.  
 *   \* `admin` - team members can pull, push and administer newly-added repositories.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Admin,
    Pull,
    Push,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Permission::Admin => "admin",
            Permission::Pull => "pull",
            Permission::Push => "push",
        }
        .fmt(f)
    }
}

impl Default for Permission {
    fn default() -> Permission {
        Permission::Pull
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub maintainers: Vec<String>,
    /**
     * The name of the team.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_team_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<TeamsCreateRequestPrivacy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repo_names: Vec<String>,
}

/**
 * The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. When a team is nested, the `privacy` for parent teams cannot be `secret`. The options are:  
 *   **For a non-nested team:**  
 *   \* `secret` - only visible to organization owners and members of this team.  
 *   \* `closed` - visible to all members of this organization.  
 *   **For a parent or child team:**  
 *   \* `closed` - visible to all members of this organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamsUpdateInOrgRequestPrivacy {
    Closed,
    Secret,
    Noop,
}

impl std::fmt::Display for TeamsUpdateInOrgRequestPrivacy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamsUpdateInOrgRequestPrivacy::Closed => "closed",
            TeamsUpdateInOrgRequestPrivacy::Secret => "secret",
            TeamsUpdateInOrgRequestPrivacy::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for TeamsUpdateInOrgRequestPrivacy {
    fn default() -> TeamsUpdateInOrgRequestPrivacy {
        TeamsUpdateInOrgRequestPrivacy::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsUpdateInOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_team_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<TeamsUpdateInOrgRequestPrivacy>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateDiscussionInOrgRequest {
    /**
     * The discussion post's body text.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /**
     * The discussion post's title.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsUpdateDiscussionInOrgRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateDiscussionCommentInOrgRequest {
    /**
     * The discussion comment's body text.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
}

/**
 * Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a team discussion comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsListTeamDiscussionCommentInOrgContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsListTeamDiscussionCommentInOrgContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsListTeamDiscussionCommentInOrgContent::PlusOne => "+1",
            ReactionsListTeamDiscussionCommentInOrgContent::MinusOne => "-1",
            ReactionsListTeamDiscussionCommentInOrgContent::Confused => "confused",
            ReactionsListTeamDiscussionCommentInOrgContent::Eyes => "eyes",
            ReactionsListTeamDiscussionCommentInOrgContent::Heart => "heart",
            ReactionsListTeamDiscussionCommentInOrgContent::Hooray => "hooray",
            ReactionsListTeamDiscussionCommentInOrgContent::Laugh => "laugh",
            ReactionsListTeamDiscussionCommentInOrgContent::Rocket => "rocket",
            ReactionsListTeamDiscussionCommentInOrgContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsListTeamDiscussionCommentInOrgContent {
    fn default() -> ReactionsListTeamDiscussionCommentInOrgContent {
        ReactionsListTeamDiscussionCommentInOrgContent::Noop
    }
}

/**
 * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsCreateTeamDiscussionCommentInOrgRequestContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsCreateTeamDiscussionCommentInOrgRequestContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::PlusOne => "+1",
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::MinusOne => "-1",
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::Confused => "confused",
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::Eyes => "eyes",
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::Heart => "heart",
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::Hooray => "hooray",
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::Laugh => "laugh",
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::Rocket => "rocket",
            ReactionsCreateTeamDiscussionCommentInOrgRequestContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsCreateTeamDiscussionCommentInOrgRequestContent {
    fn default() -> ReactionsCreateTeamDiscussionCommentInOrgRequestContent {
        ReactionsCreateTeamDiscussionCommentInOrgRequestContent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreateTeamDiscussionCommentInOrgRequest {
    /**
     * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion comment.
     */
    #[serde()]
    pub content: ReactionsCreateTeamDiscussionCommentInOrgRequestContent,
}

/**
 * Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a team discussion.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsListTeamDiscussionInOrgContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsListTeamDiscussionInOrgContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsListTeamDiscussionInOrgContent::PlusOne => "+1",
            ReactionsListTeamDiscussionInOrgContent::MinusOne => "-1",
            ReactionsListTeamDiscussionInOrgContent::Confused => "confused",
            ReactionsListTeamDiscussionInOrgContent::Eyes => "eyes",
            ReactionsListTeamDiscussionInOrgContent::Heart => "heart",
            ReactionsListTeamDiscussionInOrgContent::Hooray => "hooray",
            ReactionsListTeamDiscussionInOrgContent::Laugh => "laugh",
            ReactionsListTeamDiscussionInOrgContent::Rocket => "rocket",
            ReactionsListTeamDiscussionInOrgContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsListTeamDiscussionInOrgContent {
    fn default() -> ReactionsListTeamDiscussionInOrgContent {
        ReactionsListTeamDiscussionInOrgContent::Noop
    }
}

/**
 * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsCreateTeamDiscussionInOrgRequestContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsCreateTeamDiscussionInOrgRequestContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsCreateTeamDiscussionInOrgRequestContent::PlusOne => "+1",
            ReactionsCreateTeamDiscussionInOrgRequestContent::MinusOne => "-1",
            ReactionsCreateTeamDiscussionInOrgRequestContent::Confused => "confused",
            ReactionsCreateTeamDiscussionInOrgRequestContent::Eyes => "eyes",
            ReactionsCreateTeamDiscussionInOrgRequestContent::Heart => "heart",
            ReactionsCreateTeamDiscussionInOrgRequestContent::Hooray => "hooray",
            ReactionsCreateTeamDiscussionInOrgRequestContent::Laugh => "laugh",
            ReactionsCreateTeamDiscussionInOrgRequestContent::Rocket => "rocket",
            ReactionsCreateTeamDiscussionInOrgRequestContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsCreateTeamDiscussionInOrgRequestContent {
    fn default() -> ReactionsCreateTeamDiscussionInOrgRequestContent {
        ReactionsCreateTeamDiscussionInOrgRequestContent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreateTeamDiscussionInOrgRequest {
    /**
     * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion.
     */
    #[serde()]
    pub content: ReactionsCreateTeamDiscussionInOrgRequestContent,
}

/**
 * Filters members returned by their role in the team. Can be one of:  
 *   \* `member` - normal members of the team.  
 *   \* `maintainer` - team maintainers.  
 *   \* `all` - all members of the team.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamsListMembersInOrgRole {
    All,
    Maintainer,
    Member,
}

impl std::fmt::Display for TeamsListMembersInOrgRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamsListMembersInOrgRole::All => "all",
            TeamsListMembersInOrgRole::Maintainer => "maintainer",
            TeamsListMembersInOrgRole::Member => "member",
        }
        .fmt(f)
    }
}

impl Default for TeamsListMembersInOrgRole {
    fn default() -> TeamsListMembersInOrgRole {
        TeamsListMembersInOrgRole::All
    }
}

/**
 * The role that this user should have in the team. Can be one of:  
 *   \* `member` - a normal member of the team.  
 *   \* `maintainer` - a team maintainer. Able to add/remove other team members, promote other team members to team maintainer, and edit the team's name and description.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamsAddUpdateMembershipUserInOrgRequestRole {
    Maintainer,
    Member,
}

impl std::fmt::Display for TeamsAddUpdateMembershipUserInOrgRequestRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamsAddUpdateMembershipUserInOrgRequestRole::Maintainer => "maintainer",
            TeamsAddUpdateMembershipUserInOrgRequestRole::Member => "member",
        }
        .fmt(f)
    }
}

impl Default for TeamsAddUpdateMembershipUserInOrgRequestRole {
    fn default() -> TeamsAddUpdateMembershipUserInOrgRequestRole {
        TeamsAddUpdateMembershipUserInOrgRequestRole::Member
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateMembershipUserInOrgRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<TeamsAddUpdateMembershipUserInOrgRequestRole>,
}

/**
 * The permission to grant to the team for this project. Can be one of:  
 *   \* `read` - team members can read, but not write to or administer this project.  
 *   \* `write` - team members can read and write, but not administer this project.  
 *   \* `admin` - team members can read, write and administer this project.  
 *   Default: the team's `permission` attribute will be used to determine what permission to grant the team on this project. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamsAddUpdateProjectPermissionsInOrgRequestPermission {
    Admin,
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for TeamsAddUpdateProjectPermissionsInOrgRequestPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamsAddUpdateProjectPermissionsInOrgRequestPermission::Admin => "admin",
            TeamsAddUpdateProjectPermissionsInOrgRequestPermission::Read => "read",
            TeamsAddUpdateProjectPermissionsInOrgRequestPermission::Write => "write",
            TeamsAddUpdateProjectPermissionsInOrgRequestPermission::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for TeamsAddUpdateProjectPermissionsInOrgRequestPermission {
    fn default() -> TeamsAddUpdateProjectPermissionsInOrgRequestPermission {
        TeamsAddUpdateProjectPermissionsInOrgRequestPermission::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateProjectPermissionsInOrgRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<TeamsAddUpdateProjectPermissionsInOrgRequestPermission>,
}

/**
 * The permission to grant the team on this repository. Can be one of:  
 *   \* `pull` - team members can pull, but not push to or administer this repository.  
 *   \* `push` - team members can pull and push, but not administer this repository.  
 *   \* `admin` - team members can pull, push and administer this repository.  
 *   \* `maintain` - team members can manage the repository without access to sensitive or destructive actions. Recommended for project managers. Only applies to repositories owned by organizations.  
 *   \* `triage` - team members can proactively manage issues and pull requests without write access. Recommended for contributors who triage a repository. Only applies to repositories owned by organizations.  
 *     
 *   If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
    Admin,
    Maintain,
    Pull,
    Push,
    Triage,
    Noop,
}

impl std::fmt::Display for TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Admin => "admin",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Maintain => "maintain",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Pull => "pull",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Push => "push",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Triage => "triage",
            TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
    fn default() -> TeamsAddUpdateRepoPermissionsInOrgRequestPermission {
        TeamsAddUpdateRepoPermissionsInOrgRequestPermission::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateRepoPermissionsInOrgRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<TeamsAddUpdateRepoPermissionsInOrgRequestPermission>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateUpdateIdpGroupConnectionsInOrgRequestGroups {
    /**
     * Description of the IdP group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_description: String,
    /**
     * ID of the IdP group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_id: String,
    /**
     * Name of the IdP group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateUpdateIdpGroupConnectionsInOrgRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<TeamsCreateUpdateIdpGroupConnectionsInOrgRequestGroups>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeleteProjectsResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsUpdateCardRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsMoveCardRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_id: Option<i64>,
    /**
     * The position of the card in a column. Can be one of: `top`, `bottom`, or `after:<card_id>` to place after the specified card.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub position: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PostProjectsMoveCardResponseErrors {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub field: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub resource: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PostProjectsMoveCardResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<PostProjectsMoveCardResponseErrors>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PostProjectsMoveCardResponseErrorsData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PostProjectsCreateCardResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub code: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub documentation_url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<PostProjectsMoveCardResponseErrorsData>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsUpdateColumnRequest {
    /**
     * Name of the project column
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
 * Filters the project cards that are returned by the card's state. Can be one of `all`,`archived`, or `not_archived`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ArchivedState {
    All,
    Archived,
    NotArchived,
}

impl std::fmt::Display for ArchivedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ArchivedState::All => "all",
            ArchivedState::Archived => "archived",
            ArchivedState::NotArchived => "not_archived",
        }
        .fmt(f)
    }
}

impl Default for ArchivedState {
    fn default() -> ArchivedState {
        ArchivedState::NotArchived
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsCreateCardRequest {
    /**
     * The project card's note
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsMoveColumnRequest {
    /**
     * The position of the column in a project. Can be one of: `first`, `last`, or `after:<column_id>` to place after the specified column.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub position: String,
}

/**
 * The baseline permission that all organization members have on this project
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectsUpdateRequestOrganizationPermission {
    Admin,
    None,
    Read,
    Write,
    Noop,
}

impl std::fmt::Display for ProjectsUpdateRequestOrganizationPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProjectsUpdateRequestOrganizationPermission::Admin => "admin",
            ProjectsUpdateRequestOrganizationPermission::None => "none",
            ProjectsUpdateRequestOrganizationPermission::Read => "read",
            ProjectsUpdateRequestOrganizationPermission::Write => "write",
            ProjectsUpdateRequestOrganizationPermission::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ProjectsUpdateRequestOrganizationPermission {
    fn default() -> ProjectsUpdateRequestOrganizationPermission {
        ProjectsUpdateRequestOrganizationPermission::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_permission: Option<ProjectsUpdateRequestOrganizationPermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

/**
 * Filters the collaborators by their affiliation. Can be one of:  
 *   \* `outside`: Outside collaborators of a project that are not a member of the project's organization.  
 *   \* `direct`: Collaborators with permissions to a project, regardless of organization membership status.  
 *   \* `all`: All collaborators the authenticated user can see.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Affiliation {
    All,
    Direct,
    Outside,
}

impl std::fmt::Display for Affiliation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Affiliation::All => "all",
            Affiliation::Direct => "direct",
            Affiliation::Outside => "outside",
        }
        .fmt(f)
    }
}

impl Default for Affiliation {
    fn default() -> Affiliation {
        Affiliation::All
    }
}

/**
 * The permission to grant the collaborator.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectsAddCollaboratorRequestPermission {
    Admin,
    Read,
    Write,
}

impl std::fmt::Display for ProjectsAddCollaboratorRequestPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProjectsAddCollaboratorRequestPermission::Admin => "admin",
            ProjectsAddCollaboratorRequestPermission::Read => "read",
            ProjectsAddCollaboratorRequestPermission::Write => "write",
        }
        .fmt(f)
    }
}

impl Default for ProjectsAddCollaboratorRequestPermission {
    fn default() -> ProjectsAddCollaboratorRequestPermission {
        ProjectsAddCollaboratorRequestPermission::Write
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsAddCollaboratorRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<ProjectsAddCollaboratorRequestPermission>,
}

/**
 * Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. The `visibility` parameter overrides the `private` parameter when you use both along with the `nebula-preview` preview header.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposUpdateRequestVisibility {
    Internal,
    Private,
    Public,
    Visibility,
    Noop,
}

impl std::fmt::Display for ReposUpdateRequestVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposUpdateRequestVisibility::Internal => "internal",
            ReposUpdateRequestVisibility::Private => "private",
            ReposUpdateRequestVisibility::Public => "public",
            ReposUpdateRequestVisibility::Visibility => "visibility",
            ReposUpdateRequestVisibility::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposUpdateRequestVisibility {
    fn default() -> ReposUpdateRequestVisibility {
        ReposUpdateRequestVisibility::Noop
    }
}

/// Use the `status` property to enable or disable GitHub Advanced Security for this repository. For more information, see "[About GitHub Advanced Security](/github/getting-started-with-github/learning-about-github/about-github-advanced-security)."
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateRequestSecurityAnalysisAdvanced {
    /**
     * Use the `status` property to enable or disable GitHub Advanced Security for this repository. For more information, see "[About GitHub Advanced Security](/github/getting-started-with-github/learning-about-github/about-github-advanced-security)."
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

/// Use the `status` property to enable or disable secret scanning for this repository. For more information, see "[About secret scanning](/code-security/secret-security/about-secret-scanning)."
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateRequestSecurityAnalysisSecretScanning {
    /**
     * Use the `status` property to enable or disable secret scanning for this repository. For more information, see "[About secret scanning](/code-security/secret-security/about-secret-scanning)."
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub status: String,
}

/// Specify which security and analysis features to enable or disable. For example, to enable GitHub Advanced Security, use this data in the body of the PATCH request: `{"security_and_analysis": {"advanced_security": {"status": "enabled"}}}`. If you have admin permissions for a private repository covered by an Advanced Security license, you can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateRequestSecurityAnalysis {
    /**
     * Specify which security and analysis features to enable or disable. For example, to enable GitHub Advanced Security, use this data in the body of the PATCH request: `{"security_and_analysis": {"advanced_security": {"status": "enabled"}}}`. If you have admin permissions for a private repository covered by an Advanced Security license, you can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advanced_security: Option<ReposUpdateRequestSecurityAnalysisAdvanced>,
    /**
     * Specify which security and analysis features to enable or disable. For example, to enable GitHub Advanced Security, use this data in the body of the PATCH request: `{"security_and_analysis": {"advanced_security": {"status": "enabled"}}}`. If you have admin permissions for a private repository covered by an Advanced Security license, you can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning: Option<ReposUpdateRequestSecurityAnalysisSecretScanning>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub default_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_and_analysis: Option<ReposUpdateRequestSecurityAnalysis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ReposUpdateRequestVisibility>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListArtifactsRepoResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<Artifact>,
    #[serde(default)]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsSetGithubPermissionsRepositoryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
    /**
     * Whether GitHub Actions is enabled on the repository.
     */
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListWorkflowRunsResponse {
    #[serde(default)]
    pub total_count: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflow_runs: Vec<WorkflowRun>,
}

/**
 * Filters jobs by their `completed_at` timestamp. Can be one of:  
 *   \* `latest`: Returns jobs from the most recent execution of the workflow run.  
 *   \* `all`: Returns all jobs for a workflow run, including from old executions of the workflow run.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionsListJobsWorkflowRunFilter {
    All,
    Latest,
}

impl std::fmt::Display for ActionsListJobsWorkflowRunFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ActionsListJobsWorkflowRunFilter::All => "all",
            ActionsListJobsWorkflowRunFilter::Latest => "latest",
        }
        .fmt(f)
    }
}

impl Default for ActionsListJobsWorkflowRunFilter {
    fn default() -> ActionsListJobsWorkflowRunFilter {
        ActionsListJobsWorkflowRunFilter::Latest
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListJobsWorkflowRunResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jobs: Vec<Job>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Whether to approve or reject deployment to the specified environments. Must be one of: `approved` or `rejected`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActionsReviewPendingDeploymentsRunRequestState {
    Approved,
    Rejected,
    Noop,
}

impl std::fmt::Display for ActionsReviewPendingDeploymentsRunRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ActionsReviewPendingDeploymentsRunRequestState::Approved => "approved",
            ActionsReviewPendingDeploymentsRunRequestState::Rejected => "rejected",
            ActionsReviewPendingDeploymentsRunRequestState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ActionsReviewPendingDeploymentsRunRequestState {
    fn default() -> ActionsReviewPendingDeploymentsRunRequestState {
        ActionsReviewPendingDeploymentsRunRequestState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsReviewPendingDeploymentsRunRequest {
    /**
     * A comment to accompany the deployment review
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub comment: String,
    /**
     * The list of environment ids to approve or reject
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environment_ids: Vec<i64>,
    /**
     * Whether to approve or reject deployment to the specified environments. Must be one of: `approved` or `rejected`
     */
    #[serde()]
    pub state: ActionsReviewPendingDeploymentsRunRequestState,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListRepoSecretsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<ActionsSecret>,
    #[serde(default)]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateUpdateRepoSecretRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encrypted_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetActionsListRepoWorkflowsResponse {
    #[serde(default)]
    pub total_count: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflows: Vec<Workflow>,
}

/// Input keys and values configured in the workflow file. The maximum number of properties is 10. Any default properties configured in the workflow file will be used when `inputs` are omitted.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Inputs {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateWorkflowDispatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Inputs>,
    /**
     * The git reference for the workflow. The reference can be a branch or tag name.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
}

/// Require status checks to pass before merging. Set to `null` to disable.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateBranchProtectionRequestRequiredStatusChecks {
    /**
     * The list of status checks to require in order to merge into this branch
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
    /**
     * Require branches to be up to date before merging.
     */
    #[serde(default)]
    pub strict: bool,
}

/// Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateBranchProtectionRequestRequiredPullReviewsDismissalRestrictions {
    /**
     * Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<String>,
    /**
     * Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}

/// Require at least one approving review on a pull request, before merging. Set to `null` to disable.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateBranchProtectionRequestRequiredPullReviews {
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions:
        Option<ReposUpdateBranchProtectionRequestRequiredPullReviewsDismissalRestrictions>,
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<i64>,
}

/// Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Restrictions {
    /**
     * Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apps: Vec<String>,
    /**
     * The list of team `slug`s with push access
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<String>,
    /**
     * The list of user `login`s with push access
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateBranchProtectionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_deletions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_force_pushes: Option<bool>,
    /**
     * Enforce all configured restrictions for administrators. Set to `true` to enforce required status checks for repository administrators. Set to `null` to disable.
     */
    #[serde(default)]
    pub enforce_admins: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_conversation_resolution: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_linear_history: Option<bool>,
    /**
     * Require at least one approving review on a pull request, before merging. Set to `null` to disable.
     */
    #[serde()]
    pub required_pull_request_reviews: ReposUpdateBranchProtectionRequestRequiredPullReviews,
    /**
     * Require status checks to pass before merging. Set to `null` to disable.
     */
    #[serde()]
    pub required_status_checks: ReposUpdateBranchProtectionRequestRequiredStatusChecks,
    /**
     * Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
     */
    #[serde()]
    pub restrictions: Restrictions,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdatePullRequestReviewProtection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissal_restrictions:
        Option<ReposUpdateBranchProtectionRequestRequiredPullReviewsDismissalRestrictions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_approving_review_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateStatusCheckProtectionRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddStatusCheckContextsRequest {
    /**
     * contexts parameter
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddAppAccessRestrictionsRequest {
    /**
     * apps parameter
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddTeamAccessRestrictionsRequest {
    /**
     * teams parameter
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub teams: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddUserAccessRestrictionsRequest {
    /**
     * users parameter
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposRenameBranchRequest {
    /**
     * The new name of the branch.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub new_name: String,
}

/**
 * The current status. Can be one of `queued`, `in_progress`, or `completed`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChecksCreateRequestStatus {
    Completed,
    InProgress,
    Queued,
}

impl std::fmt::Display for ChecksCreateRequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChecksCreateRequestStatus::Completed => "completed",
            ChecksCreateRequestStatus::InProgress => "in_progress",
            ChecksCreateRequestStatus::Queued => "queued",
        }
        .fmt(f)
    }
}

impl Default for ChecksCreateRequestStatus {
    fn default() -> ChecksCreateRequestStatus {
        ChecksCreateRequestStatus::Queued
    }
}

/**
 * **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`. When the conclusion is `action_required`, additional details should be provided on the site specified by `details_url`.  
 *   **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChecksCreateRequestConclusion {
    ActionRequired,
    Cancelled,
    Failure,
    Neutral,
    Skipped,
    Stale,
    Success,
    TimedOut,
    Noop,
}

impl std::fmt::Display for ChecksCreateRequestConclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChecksCreateRequestConclusion::ActionRequired => "action_required",
            ChecksCreateRequestConclusion::Cancelled => "cancelled",
            ChecksCreateRequestConclusion::Failure => "failure",
            ChecksCreateRequestConclusion::Neutral => "neutral",
            ChecksCreateRequestConclusion::Skipped => "skipped",
            ChecksCreateRequestConclusion::Stale => "stale",
            ChecksCreateRequestConclusion::Success => "success",
            ChecksCreateRequestConclusion::TimedOut => "timed_out",
            ChecksCreateRequestConclusion::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ChecksCreateRequestConclusion {
    fn default() -> ChecksCreateRequestConclusion {
        ChecksCreateRequestConclusion::Noop
    }
}

/**
 * The level of the annotation. Can be one of `notice`, `warning`, or `failure`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationLevel {
    Failure,
    Notice,
    Warning,
    Noop,
}

impl std::fmt::Display for AnnotationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AnnotationLevel::Failure => "failure",
            AnnotationLevel::Notice => "notice",
            AnnotationLevel::Warning => "warning",
            AnnotationLevel::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for AnnotationLevel {
    fn default() -> AnnotationLevel {
        AnnotationLevel::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Annotations {
    /**
     * The level of the annotation. Can be one of `notice`, `warning`, or `failure`.
     */
    #[serde()]
    pub annotation_level: AnnotationLevel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i64>,
    /**
     * The end line of the annotation.
     */
    #[serde(default)]
    pub end_line: i64,
    /**
     * A short description of the feedback for these lines of code. The maximum size is 64 KB.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * The path of the file to add an annotation to. For example, `assets/css/main.css`.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub raw_details: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i64>,
    /**
     * The start line of the annotation.
     */
    #[serde(default)]
    pub start_line: i64,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Images {
    /**
     * The alternative text for the image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub alt: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub caption: String,
    /**
     * The full URL of the image.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub image_url: String,
}

/// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksCreateRequestOutput {
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotations>,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Images>,
    /**
     * The summary of the check run. This parameter supports Markdown.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
    /**
     * The title of the check run.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksCreateRequestActions {
    /**
     * A short explanation of what this action would do. The maximum size is 40 characters.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * A reference for the action on the integrator's system. The maximum size is 20 characters.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub identifier: String,
    /**
     * The text to be displayed on a button in the web UI. The maximum size is 20 characters.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksCreateRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ChecksCreateRequestActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<ChecksCreateRequestConclusion>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub details_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    /**
     * The SHA of the commit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
    /**
     * The name of the check. For example, "code-coverage".
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<ChecksCreateRequestOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ChecksCreateRequestStatus>,
}

/**
 * **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`.  
 *   **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChecksUpdateRequestConclusion {
    ActionRequired,
    Cancelled,
    Failure,
    Neutral,
    Skipped,
    Stale,
    Success,
    TimedOut,
    Noop,
}

impl std::fmt::Display for ChecksUpdateRequestConclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChecksUpdateRequestConclusion::ActionRequired => "action_required",
            ChecksUpdateRequestConclusion::Cancelled => "cancelled",
            ChecksUpdateRequestConclusion::Failure => "failure",
            ChecksUpdateRequestConclusion::Neutral => "neutral",
            ChecksUpdateRequestConclusion::Skipped => "skipped",
            ChecksUpdateRequestConclusion::Stale => "stale",
            ChecksUpdateRequestConclusion::Success => "success",
            ChecksUpdateRequestConclusion::TimedOut => "timed_out",
            ChecksUpdateRequestConclusion::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ChecksUpdateRequestConclusion {
    fn default() -> ChecksUpdateRequestConclusion {
        ChecksUpdateRequestConclusion::Noop
    }
}

/// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksUpdateRequestOutput {
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<Annotations>,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Images>,
    /**
     * Can contain Markdown.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub text: String,
    /**
     * Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksUpdateRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ChecksCreateRequestActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<ChecksUpdateRequestConclusion>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub details_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<ChecksUpdateRequestOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ChecksCreateRequestStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksCreateSuiteRequest {
    /**
     * The sha of the head commit.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head_sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksSetSuitesPreferencesRequestAutoTrigger {
    /**
     * The `id` of the GitHub App.
     */
    #[serde(default)]
    pub app_id: i64,
    /**
     * Set to `true` to enable automatic creation of CheckSuite events upon pushes to the repository, or `false` to disable them.
     */
    #[serde(default)]
    pub setting: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ChecksSetSuitesPreferencesRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auto_trigger_checks: Vec<ChecksSetSuitesPreferencesRequestAutoTrigger>,
}

/**
 * Filters check runs by their `completed_at` timestamp. Can be one of `latest` (returning the most recent check runs) or `all`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChecksListSuiteFilter {
    All,
    Latest,
}

impl std::fmt::Display for ChecksListSuiteFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChecksListSuiteFilter::All => "all",
            ChecksListSuiteFilter::Latest => "latest",
        }
        .fmt(f)
    }
}

impl Default for ChecksListSuiteFilter {
    fn default() -> ChecksListSuiteFilter {
        ChecksListSuiteFilter::Latest
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetChecksListRefResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub check_runs: Vec<CheckRun>,
    #[serde(default)]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningUpdateAlertRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_reason: Option<serde_json::Value>,
    /**
     * Sets the state of the code scanning alert. Can be one of `open` or `dismissed`. You must provide `dismissed_reason` when you set the state to `dismissed`.
     */
    #[serde()]
    pub state: CodeScanningAlertSetState,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CodeScanningUploadSarifRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub checkout_uri: String,
    /**
     * The SHA of the commit to which the analysis you are uploading relates.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_sha: String,
    /**
     * The full Git reference, formatted as `refs/heads/<branch name>`,
     *  `refs/pull/<number>/merge`, or `refs/pull/<number>/head`.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    /**
     * A Base64 string representing the SARIF file to upload. You must first compress your SARIF file using [`gzip`](http://www.gnu.org/software/gzip/manual/gzip.html) and then translate the contents of the file into a Base64 encoding string. For more information, see "[SARIF support for code scanning](https://docs.github.com/code-security/secure-coding/sarif-support-for-code-scanning)."
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sarif: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tool_name: String,
}

/**
 * Filter collaborators returned by their affiliation. Can be one of:  
 *   \* `outside`: All outside collaborators of an organization-owned repository.  
 *   \* `direct`: All collaborators with permissions to an organization-owned repository, regardless of organization membership status.  
 *   \* `all`: All collaborators the authenticated user can see.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListCollaboratorsAffiliation {
    All,
    Direct,
    Outside,
}

impl std::fmt::Display for ReposListCollaboratorsAffiliation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListCollaboratorsAffiliation::All => "all",
            ReposListCollaboratorsAffiliation::Direct => "direct",
            ReposListCollaboratorsAffiliation::Outside => "outside",
        }
        .fmt(f)
    }
}

impl Default for ReposListCollaboratorsAffiliation {
    fn default() -> ReposListCollaboratorsAffiliation {
        ReposListCollaboratorsAffiliation::All
    }
}

/**
 * The permission to grant the collaborator. **Only valid on organization-owned repositories.** Can be one of:  
 *   \* `pull` - can pull, but not push to or administer this repository.  
 *   \* `push` - can pull and push, but not administer this repository.  
 *   \* `admin` - can pull, push and administer this repository.  
 *   \* `maintain` - Recommended for project managers who need to manage the repository without access to sensitive or destructive actions.  
 *   \* `triage` - Recommended for contributors who need to proactively manage issues and pull requests without write access.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposAddCollaboratorRequestPermission {
    Admin,
    Maintain,
    Pull,
    Push,
    Triage,
}

impl std::fmt::Display for ReposAddCollaboratorRequestPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposAddCollaboratorRequestPermission::Admin => "admin",
            ReposAddCollaboratorRequestPermission::Maintain => "maintain",
            ReposAddCollaboratorRequestPermission::Pull => "pull",
            ReposAddCollaboratorRequestPermission::Push => "push",
            ReposAddCollaboratorRequestPermission::Triage => "triage",
        }
        .fmt(f)
    }
}

impl Default for ReposAddCollaboratorRequestPermission {
    fn default() -> ReposAddCollaboratorRequestPermission {
        ReposAddCollaboratorRequestPermission::Push
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposAddCollaboratorRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<ReposAddCollaboratorRequestPermission>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub permissions: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateCommitCommentRequest {
    /**
     * The contents of the comment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
}

/**
 * Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a commit comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsListCommitCommentContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsListCommitCommentContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsListCommitCommentContent::PlusOne => "+1",
            ReactionsListCommitCommentContent::MinusOne => "-1",
            ReactionsListCommitCommentContent::Confused => "confused",
            ReactionsListCommitCommentContent::Eyes => "eyes",
            ReactionsListCommitCommentContent::Heart => "heart",
            ReactionsListCommitCommentContent::Hooray => "hooray",
            ReactionsListCommitCommentContent::Laugh => "laugh",
            ReactionsListCommitCommentContent::Rocket => "rocket",
            ReactionsListCommitCommentContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsListCommitCommentContent {
    fn default() -> ReactionsListCommitCommentContent {
        ReactionsListCommitCommentContent::Noop
    }
}

/**
 * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the commit comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsCreateCommitCommentRequestContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsCreateCommitCommentRequestContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsCreateCommitCommentRequestContent::PlusOne => "+1",
            ReactionsCreateCommitCommentRequestContent::MinusOne => "-1",
            ReactionsCreateCommitCommentRequestContent::Confused => "confused",
            ReactionsCreateCommitCommentRequestContent::Eyes => "eyes",
            ReactionsCreateCommitCommentRequestContent::Heart => "heart",
            ReactionsCreateCommitCommentRequestContent::Hooray => "hooray",
            ReactionsCreateCommitCommentRequestContent::Laugh => "laugh",
            ReactionsCreateCommitCommentRequestContent::Rocket => "rocket",
            ReactionsCreateCommitCommentRequestContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsCreateCommitCommentRequestContent {
    fn default() -> ReactionsCreateCommitCommentRequestContent {
        ReactionsCreateCommitCommentRequestContent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreateCommitCommentRequest {
    /**
     * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the commit comment.
     */
    #[serde()]
    pub content: ReactionsCreateCommitCommentRequestContent,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateCommitCommentRequest {
    /**
     * The contents of the comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetChecksListSuitesRefResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub check_suites: Vec<CheckSuite>,
    #[serde(default)]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppsCreateContentAttachmentRequest {
    /**
     * The body of the attachment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * The title of the attachment
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/// The person that committed the file. Default: the authenticated user.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUpdateFileContentsRequestCommitter {
    /**
     * The person that committed the file. Default: the authenticated user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The email of the author or committer of the commit. You'll receive a `422` status code if `email` is omitted.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The name of the author or committer of the commit. You'll receive a `422` status code if `name` is omitted.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// The author of the file. Default: The `committer` or the authenticated user if you omit `committer`.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUpdateFileContentsRequestAuthor {
    /**
     * The author of the file. Default: The `committer` or the authenticated user if you omit `committer`.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub date: String,
    /**
     * The email of the author or committer of the commit. You'll receive a `422` status code if `email` is omitted.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The name of the author or committer of the commit. You'll receive a `422` status code if `name` is omitted.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUpdateFileContentsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<ReposCreateUpdateFileContentsRequestAuthor>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<ReposCreateUpdateFileContentsRequestCommitter>,
    /**
     * The new file content, using Base64 encoding.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    /**
     * The commit message.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

/// object containing information about the committer.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposDeleteFileRequestCommitter {
    /**
     * object containing information about the committer.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * object containing information about the committer.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// object containing information about the author.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposDeleteFileRequestAuthor {
    /**
     * object containing information about the author.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * object containing information about the author.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposDeleteFileRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<ReposDeleteFileRequestAuthor>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<ReposDeleteFileRequestCommitter>,
    /**
     * The commit message.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * The blob SHA of the file being replaced.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateDeploymentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Data>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    /**
     * The ref to deploy. This can be a branch, tag, or SHA.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_contexts: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub task: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
}

/**
 * The state of the status. Can be one of `error`, `failure`, `inactive`, `in_progress`, `queued` `pending`, or `success`. **Note:** To use the `inactive` state, you must provide the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. To use the `in_progress` and `queued` states, you must provide the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposCreateDeploymentStatusRequestState {
    Error,
    Failure,
    InProgress,
    Inactive,
    Pending,
    Queued,
    Success,
    Noop,
}

impl std::fmt::Display for ReposCreateDeploymentStatusRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposCreateDeploymentStatusRequestState::Error => "error",
            ReposCreateDeploymentStatusRequestState::Failure => "failure",
            ReposCreateDeploymentStatusRequestState::InProgress => "in_progress",
            ReposCreateDeploymentStatusRequestState::Inactive => "inactive",
            ReposCreateDeploymentStatusRequestState::Pending => "pending",
            ReposCreateDeploymentStatusRequestState::Queued => "queued",
            ReposCreateDeploymentStatusRequestState::Success => "success",
            ReposCreateDeploymentStatusRequestState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposCreateDeploymentStatusRequestState {
    fn default() -> ReposCreateDeploymentStatusRequestState {
        ReposCreateDeploymentStatusRequestState::Noop
    }
}

/**
 * Name for the target deployment environment, which can be changed when setting a deploy status. For example, `production`, `staging`, or `qa`. **Note:** This parameter requires you to use the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposCreateDeploymentStatusRequestEnvironment {
    Production,
    Qa,
    Staging,
    Noop,
}

impl std::fmt::Display for ReposCreateDeploymentStatusRequestEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposCreateDeploymentStatusRequestEnvironment::Production => "production",
            ReposCreateDeploymentStatusRequestEnvironment::Qa => "qa",
            ReposCreateDeploymentStatusRequestEnvironment::Staging => "staging",
            ReposCreateDeploymentStatusRequestEnvironment::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposCreateDeploymentStatusRequestEnvironment {
    fn default() -> ReposCreateDeploymentStatusRequestEnvironment {
        ReposCreateDeploymentStatusRequestEnvironment::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateDeploymentStatusRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_inactive: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ReposCreateDeploymentStatusRequestEnvironment>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub environment_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub log_url: String,
    /**
     * The state of the status. Can be one of `error`, `failure`, `inactive`, `in_progress`, `queued` `pending`, or `success`. **Note:** To use the `inactive` state, you must provide the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. To use the `in_progress` and `queued` states, you must provide the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub.
     */
    #[serde()]
    pub state: ReposCreateDeploymentStatusRequestState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
}

/// JSON payload with extra information about the webhook event that your action or worklow may use.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ClientPayload {}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateDispatchEventRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_payload: Option<ClientPayload>,
    /**
     * A custom webhook event name.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetReposAllEnvironmentsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environments: Vec<Environment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUpdateEnvironmentRequestReviewers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<DeploymentReviewerType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUpdateEnvironmentRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<ReposCreateUpdateEnvironmentRequestReviewers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_timer: Option<i64>,
}

/**
 * The sort order. Can be either `newest`, `oldest`, or `stargazers`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListForksSort {
    Newest,
    Oldest,
    Stargazers,
    Watchers,
}

impl std::fmt::Display for ReposListForksSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListForksSort::Newest => "newest",
            ReposListForksSort::Oldest => "oldest",
            ReposListForksSort::Stargazers => "stargazers",
            ReposListForksSort::Watchers => "watchers",
        }
        .fmt(f)
    }
}

impl Default for ReposListForksSort {
    fn default() -> ReposListForksSort {
        ReposListForksSort::Newest
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateForkRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub organization: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateBlobRequest {
    /**
     * The new blob's content.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encoding: String,
}

/// Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateCommitRequestAuthor {
    /**
     * Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The email of the author (or committer) of the commit
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The name of the author (or committer) of the commit
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateCommitRequestCommitter {
    /**
     * Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateCommitRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<GitCreateCommitRequestAuthor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<GitCreateCommitRequestCommitter>,
    /**
     * The commit message
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub signature: String,
    /**
     * The SHA of the tree object this commit points to
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tree: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateRefRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    /**
     * The name of the fully qualified reference (ie: `refs/heads/master`). If it doesn't start with 'refs' and have at least two slashes, it will be rejected.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "ref"
    )]
    pub ref_: String,
    /**
     * The SHA1 value for this reference.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitUpdateRefRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /**
     * The SHA1 value to set this reference to
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

/**
 * The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitCreateTagRequestType {
    Blob,
    Commit,
    Tree,
    Noop,
}

impl std::fmt::Display for GitCreateTagRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GitCreateTagRequestType::Blob => "blob",
            GitCreateTagRequestType::Commit => "commit",
            GitCreateTagRequestType::Tree => "tree",
            GitCreateTagRequestType::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for GitCreateTagRequestType {
    fn default() -> GitCreateTagRequestType {
        GitCreateTagRequestType::Noop
    }
}

/// An object with information about the individual creating the tag.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateTagRequestTagger {
    /**
     * An object with information about the individual creating the tag.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    /**
     * The email of the author of the tag
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    /**
     * The name of the author of the tag
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateTagRequest {
    /**
     * The tag message.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    /**
     * The SHA of the git object this is tagging.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub object: String,
    /**
     * The tag's name. This is typically a version (e.g., "v0.0.1").
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagger: Option<GitCreateTagRequestTagger>,
    /**
     * The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
     */
    #[serde(rename = "type")]
    pub type_: GitCreateTagRequestType,
}

/**
 * The file mode; one of `100644` for file (blob), `100755` for executable (blob), `040000` for subdirectory (tree), `160000` for submodule (commit), or `120000` for a blob that specifies the path of a symlink.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitCreateTreeRequestMode {
    SubdirectoryTree,
    FileBlob,
    ExecutableBlob,
    SymlinkPathBlob,
    SubmoduleCommit,
    Noop,
}

impl std::fmt::Display for GitCreateTreeRequestMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GitCreateTreeRequestMode::SubdirectoryTree => "040000",
            GitCreateTreeRequestMode::FileBlob => "100644",
            GitCreateTreeRequestMode::ExecutableBlob => "100755",
            GitCreateTreeRequestMode::SymlinkPathBlob => "120000",
            GitCreateTreeRequestMode::SubmoduleCommit => "160000",
            GitCreateTreeRequestMode::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for GitCreateTreeRequestMode {
    fn default() -> GitCreateTreeRequestMode {
        GitCreateTreeRequestMode::Noop
    }
}

/**
 * Either `blob`, `tree`, or `commit`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GitCreateTreeRequestType {
    Blob,
    Commit,
    Tree,
    Noop,
}

impl std::fmt::Display for GitCreateTreeRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GitCreateTreeRequestType::Blob => "blob",
            GitCreateTreeRequestType::Commit => "commit",
            GitCreateTreeRequestType::Tree => "tree",
            GitCreateTreeRequestType::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for GitCreateTreeRequestType {
    fn default() -> GitCreateTreeRequestType {
        GitCreateTreeRequestType::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateTreeRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GitCreateTreeRequestMode>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<GitCreateTreeRequestType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GitCreateTreeRequestData {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub base_tree: String,
    /**
     * Objects (of `path`, `mode`, `type`, and `sha`) specifying a tree structure.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tree: Vec<GitCreateTreeRequest>,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateWebhookRequestConfig {
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub digest: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<ReposCreateWebhookRequestConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateWebhookRequestConfig {
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub address: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub content_type: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub insecure_ssl: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub room: String,
    /**
     * Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub secret: String,
    /**
     * The URL to which the payloads will be delivered.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateWebhookRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<ReposUpdateWebhookRequestConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_events: Vec<String>,
}

/**
 * The originating VCS type. Can be one of `subversion`, `git`, `mercurial`, or `tfvc`. Please be aware that without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Vcs {
    Git,
    Mercurial,
    Subversion,
    Tfvc,
    Noop,
}

impl std::fmt::Display for Vcs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Vcs::Git => "git",
            Vcs::Mercurial => "mercurial",
            Vcs::Subversion => "subversion",
            Vcs::Tfvc => "tfvc",
            Vcs::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for Vcs {
    fn default() -> Vcs {
        Vcs::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsStartImportRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tfvc_project: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcs: Option<Vcs>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_password: String,
    /**
     * The URL of the originating repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_url: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsUpdateImportRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tfvc_project: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_password: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub vcs_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsMapCommitAuthorRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
 * Can be one of `opt_in` (large files will be stored using Git LFS) or `opt_out` (large files will be removed during the import).
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UseLfs {
    OptIn,
    OptOut,
    Noop,
}

impl std::fmt::Display for UseLfs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UseLfs::OptIn => "opt_in",
            UseLfs::OptOut => "opt_out",
            UseLfs::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for UseLfs {
    fn default() -> UseLfs {
        UseLfs::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsSetLfsPreferenceRequest {
    /**
     * Can be one of `opt_in` (large files will be stored using Git LFS) or `opt_out` (large files will be removed during the import).
     */
    #[serde()]
    pub use_lfs: UseLfs,
}

/**
 * The permissions that the associated user will have on the repository. Valid values are `read`, `write`, `maintain`, `triage`, and `admin`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposUpdateInvitationRequestPermissions {
    Admin,
    Maintain,
    Read,
    Triage,
    Write,
    Noop,
}

impl std::fmt::Display for ReposUpdateInvitationRequestPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposUpdateInvitationRequestPermissions::Admin => "admin",
            ReposUpdateInvitationRequestPermissions::Maintain => "maintain",
            ReposUpdateInvitationRequestPermissions::Read => "read",
            ReposUpdateInvitationRequestPermissions::Triage => "triage",
            ReposUpdateInvitationRequestPermissions::Write => "write",
            ReposUpdateInvitationRequestPermissions::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposUpdateInvitationRequestPermissions {
    fn default() -> ReposUpdateInvitationRequestPermissions {
        ReposUpdateInvitationRequestPermissions::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateInvitationRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ReposUpdateInvitationRequestPermissions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignee: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestone: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/**
 * Either `asc` or `desc`. Ignored without the `sort` parameter.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssuesListCommentsRepoDirection {
    Asc,
    Desc,
    Noop,
}

impl std::fmt::Display for IssuesListCommentsRepoDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IssuesListCommentsRepoDirection::Asc => "asc",
            IssuesListCommentsRepoDirection::Desc => "desc",
            IssuesListCommentsRepoDirection::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for IssuesListCommentsRepoDirection {
    fn default() -> IssuesListCommentsRepoDirection {
        IssuesListCommentsRepoDirection::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesUpdateCommentRequest {
    /**
     * The contents of the comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
}

/**
 * Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to an issue comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsListIssueCommentContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsListIssueCommentContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsListIssueCommentContent::PlusOne => "+1",
            ReactionsListIssueCommentContent::MinusOne => "-1",
            ReactionsListIssueCommentContent::Confused => "confused",
            ReactionsListIssueCommentContent::Eyes => "eyes",
            ReactionsListIssueCommentContent::Heart => "heart",
            ReactionsListIssueCommentContent::Hooray => "hooray",
            ReactionsListIssueCommentContent::Laugh => "laugh",
            ReactionsListIssueCommentContent::Rocket => "rocket",
            ReactionsListIssueCommentContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsListIssueCommentContent {
    fn default() -> ReactionsListIssueCommentContent {
        ReactionsListIssueCommentContent::Noop
    }
}

/**
 * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the issue comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsCreateIssueCommentRequestContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsCreateIssueCommentRequestContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsCreateIssueCommentRequestContent::PlusOne => "+1",
            ReactionsCreateIssueCommentRequestContent::MinusOne => "-1",
            ReactionsCreateIssueCommentRequestContent::Confused => "confused",
            ReactionsCreateIssueCommentRequestContent::Eyes => "eyes",
            ReactionsCreateIssueCommentRequestContent::Heart => "heart",
            ReactionsCreateIssueCommentRequestContent::Hooray => "hooray",
            ReactionsCreateIssueCommentRequestContent::Laugh => "laugh",
            ReactionsCreateIssueCommentRequestContent::Rocket => "rocket",
            ReactionsCreateIssueCommentRequestContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsCreateIssueCommentRequestContent {
    fn default() -> ReactionsCreateIssueCommentRequestContent {
        ReactionsCreateIssueCommentRequestContent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreateIssueCommentRequest {
    /**
     * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the issue comment.
     */
    #[serde()]
    pub content: ReactionsCreateIssueCommentRequestContent,
}

/**
 * State of the issue. Either `open` or `closed`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssuesUpdateRequestState {
    Closed,
    Open,
    Noop,
}

impl std::fmt::Display for IssuesUpdateRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IssuesUpdateRequestState::Closed => "closed",
            IssuesUpdateRequestState::Open => "open",
            IssuesUpdateRequestState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for IssuesUpdateRequestState {
    fn default() -> IssuesUpdateRequestState {
        IssuesUpdateRequestState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub assignee: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub milestone: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IssuesUpdateRequestState>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesAddAssigneesRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesRemoveAssigneesRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesAddLabelsRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
}

/**
 * The reason for locking the issue or pull request conversation. Lock will fail if you don't use one of these reasons:  
 *   \* `off-topic`  
 *   \* `too heated`  
 *   \* `resolved`  
 *   \* `spam`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LockReason {
    OffTopic,
    Resolved,
    Spam,
    TooHeated,
    Noop,
}

impl std::fmt::Display for LockReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LockReason::OffTopic => "off-topic",
            LockReason::Resolved => "resolved",
            LockReason::Spam => "spam",
            LockReason::TooHeated => "too heated",
            LockReason::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for LockReason {
    fn default() -> LockReason {
        LockReason::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesLockRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<LockReason>,
}

/**
 * Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to an issue.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsListIssueContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsListIssueContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsListIssueContent::PlusOne => "+1",
            ReactionsListIssueContent::MinusOne => "-1",
            ReactionsListIssueContent::Confused => "confused",
            ReactionsListIssueContent::Eyes => "eyes",
            ReactionsListIssueContent::Heart => "heart",
            ReactionsListIssueContent::Hooray => "hooray",
            ReactionsListIssueContent::Laugh => "laugh",
            ReactionsListIssueContent::Rocket => "rocket",
            ReactionsListIssueContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsListIssueContent {
    fn default() -> ReactionsListIssueContent {
        ReactionsListIssueContent::Noop
    }
}

/**
 * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the issue.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsCreateIssueRequestContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsCreateIssueRequestContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsCreateIssueRequestContent::PlusOne => "+1",
            ReactionsCreateIssueRequestContent::MinusOne => "-1",
            ReactionsCreateIssueRequestContent::Confused => "confused",
            ReactionsCreateIssueRequestContent::Eyes => "eyes",
            ReactionsCreateIssueRequestContent::Heart => "heart",
            ReactionsCreateIssueRequestContent::Hooray => "hooray",
            ReactionsCreateIssueRequestContent::Laugh => "laugh",
            ReactionsCreateIssueRequestContent::Rocket => "rocket",
            ReactionsCreateIssueRequestContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsCreateIssueRequestContent {
    fn default() -> ReactionsCreateIssueRequestContent {
        ReactionsCreateIssueRequestContent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreateIssueRequest {
    /**
     * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the issue.
     */
    #[serde()]
    pub content: ReactionsCreateIssueRequestContent,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateDeployKeyRequest {
    /**
     * The contents of the key.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesCreateLabelRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The name of the label. Emoji can be added to label names, using either native emoji or colon-style markup. For example, typing `:strawberry:` will render the emoji ![:strawberry:](https://github.githubassets.com/images/icons/emoji/unicode/1f353.png ":strawberry:"). For a full list of available emoji and codes, see "[Emoji cheat sheet](https://github.com/ikatyang/emoji-cheat-sheet)."
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesUpdateLabelRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub color: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub new_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposMergeRequest {
    /**
     * The name of the base branch that the head will be merged into.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub base: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_message: String,
    /**
     * The head to merge. This can be a branch name or a commit SHA1.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head: String,
}

/**
 * The state of the milestone. Either `open`, `closed`, or `all`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssuesListMilestonesState {
    All,
    Closed,
    Open,
}

impl std::fmt::Display for IssuesListMilestonesState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IssuesListMilestonesState::All => "all",
            IssuesListMilestonesState::Closed => "closed",
            IssuesListMilestonesState::Open => "open",
        }
        .fmt(f)
    }
}

impl Default for IssuesListMilestonesState {
    fn default() -> IssuesListMilestonesState {
        IssuesListMilestonesState::Open
    }
}

/**
 * What to sort results by. Either `due_on` or `completeness`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssuesListMilestonesSort {
    Completeness,
    DueOn,
}

impl std::fmt::Display for IssuesListMilestonesSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IssuesListMilestonesSort::Completeness => "completeness",
            IssuesListMilestonesSort::DueOn => "due_on",
        }
        .fmt(f)
    }
}

impl Default for IssuesListMilestonesSort {
    fn default() -> IssuesListMilestonesSort {
        IssuesListMilestonesSort::DueOn
    }
}

/**
 * The direction of the sort. Either `asc` or `desc`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssuesListMilestonesDirection {
    Asc,
    Desc,
}

impl std::fmt::Display for IssuesListMilestonesDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IssuesListMilestonesDirection::Asc => "asc",
            IssuesListMilestonesDirection::Desc => "desc",
        }
        .fmt(f)
    }
}

impl Default for IssuesListMilestonesDirection {
    fn default() -> IssuesListMilestonesDirection {
        IssuesListMilestonesDirection::Asc
    }
}

/**
 * The state of the milestone. Either `open` or `closed`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssuesCreateMilestoneRequestState {
    Closed,
    Open,
}

impl std::fmt::Display for IssuesCreateMilestoneRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IssuesCreateMilestoneRequestState::Closed => "closed",
            IssuesCreateMilestoneRequestState::Open => "open",
        }
        .fmt(f)
    }
}

impl Default for IssuesCreateMilestoneRequestState {
    fn default() -> IssuesCreateMilestoneRequestState {
        IssuesCreateMilestoneRequestState::Open
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesCreateMilestoneRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_on: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IssuesCreateMilestoneRequestState>,
    /**
     * The title of the milestone.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct IssuesUpdateMilestoneRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_on: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IssuesCreateMilestoneRequestState>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActivityMarkRepoNotificationsAsReadRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PutPullsUpdateBranchResponse {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub url: String,
}

/**
 * The repository directory that includes the source files for the Pages site. Allowed paths are `/` or `/docs`. Default: `/`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Path {
    Root,
    Docs,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Path::Root => "/",
            Path::Docs => "/docs",
        }
        .fmt(f)
    }
}

impl Default for Path {
    fn default() -> Path {
        Path::Root
    }
}

/// The source branch and directory used to publish your Pages site.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreatePagesSiteRequestSource {
    /**
     * The repository branch used to publish your site's source files.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub branch: String,
    /**
     * The source branch and directory used to publish your Pages site.
     */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Path>,
}

/// The source branch and directory used to publish your Pages site.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreatePagesSiteRequest {
    /**
     * The source branch and directory used to publish your Pages site.
     */
    #[serde()]
    pub source: ReposCreatePagesSiteRequestSource,
}

/**
 * Update the source for the repository. Must include the branch name, and may optionally specify the subdirectory `/docs`. Possible values are `"gh-pages"`, `"master"`, and `"master /docs"`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SourceData {
    GhPages,
    Master,
    MasterDocs,
    Noop,
}

impl std::fmt::Display for SourceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SourceData::GhPages => "gh-pages",
            SourceData::Master => "master",
            SourceData::MasterDocs => "master /docs",
            SourceData::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SourceData {
    fn default() -> SourceData {
        SourceData::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateInformationAboutPagesSiteRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub cname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_enforced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceData>,
}

/**
 * Either `open`, `closed`, or `all` to filter by state.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsListState {
    All,
    Closed,
    Open,
}

impl std::fmt::Display for PullsListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsListState::All => "all",
            PullsListState::Closed => "closed",
            PullsListState::Open => "open",
        }
        .fmt(f)
    }
}

impl Default for PullsListState {
    fn default() -> PullsListState {
        PullsListState::Open
    }
}

/**
 * What to sort results by. Can be either `created`, `updated`, `popularity` (comment count) or `long-running` (age, filtering by pulls updated in the last month).
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsListSort {
    Created,
    LongRunning,
    Popularity,
    Updated,
}

impl std::fmt::Display for PullsListSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsListSort::Created => "created",
            PullsListSort::LongRunning => "long-running",
            PullsListSort::Popularity => "popularity",
            PullsListSort::Updated => "updated",
        }
        .fmt(f)
    }
}

impl Default for PullsListSort {
    fn default() -> PullsListSort {
        PullsListSort::Created
    }
}

/**
 * The direction of the sort. Can be either `asc` or `desc`. Default: `desc` when sort is `created` or sort is not specified, otherwise `asc`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsListDirection {
    Asc,
    Desc,
    Noop,
}

impl std::fmt::Display for PullsListDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsListDirection::Asc => "asc",
            PullsListDirection::Desc => "desc",
            PullsListDirection::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullsListDirection {
    fn default() -> PullsListDirection {
        PullsListDirection::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsCreateRequest {
    /**
     * The name of the branch you want the changes pulled into. This should be an existing branch on the current repository. You cannot submit a pull request to one repository that requests a merge to a base of another repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub base: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    /**
     * The name of the branch where your changes are implemented. For cross-repository pull requests in the same network, namespace `head` with a user like this: `username:branch`.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub head: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsListReviewCommentsRepoSort {
    Created,
    CreatedAt,
    Updated,
    Noop,
}

impl std::fmt::Display for PullsListReviewCommentsRepoSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsListReviewCommentsRepoSort::Created => "created",
            PullsListReviewCommentsRepoSort::CreatedAt => "created_at",
            PullsListReviewCommentsRepoSort::Updated => "updated",
            PullsListReviewCommentsRepoSort::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullsListReviewCommentsRepoSort {
    fn default() -> PullsListReviewCommentsRepoSort {
        PullsListReviewCommentsRepoSort::Noop
    }
}

/**
 * Can be either `asc` or `desc`. Ignored without `sort` parameter.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsListReviewCommentsRepoDirection {
    Asc,
    Desc,
    Noop,
}

impl std::fmt::Display for PullsListReviewCommentsRepoDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsListReviewCommentsRepoDirection::Asc => "asc",
            PullsListReviewCommentsRepoDirection::Desc => "desc",
            PullsListReviewCommentsRepoDirection::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullsListReviewCommentsRepoDirection {
    fn default() -> PullsListReviewCommentsRepoDirection {
        PullsListReviewCommentsRepoDirection::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsUpdateReviewCommentRequest {
    /**
     * The text of the reply to the review comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
}

/**
 * Returns a single [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types). Omit this parameter to list all reactions to a pull request review comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsListPullRequestReviewCommentContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsListPullRequestReviewCommentContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsListPullRequestReviewCommentContent::PlusOne => "+1",
            ReactionsListPullRequestReviewCommentContent::MinusOne => "-1",
            ReactionsListPullRequestReviewCommentContent::Confused => "confused",
            ReactionsListPullRequestReviewCommentContent::Eyes => "eyes",
            ReactionsListPullRequestReviewCommentContent::Heart => "heart",
            ReactionsListPullRequestReviewCommentContent::Hooray => "hooray",
            ReactionsListPullRequestReviewCommentContent::Laugh => "laugh",
            ReactionsListPullRequestReviewCommentContent::Rocket => "rocket",
            ReactionsListPullRequestReviewCommentContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsListPullRequestReviewCommentContent {
    fn default() -> ReactionsListPullRequestReviewCommentContent {
        ReactionsListPullRequestReviewCommentContent::Noop
    }
}

/**
 * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the pull request review comment.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsCreatePullRequestReviewCommentContent {
    PlusOne,
    MinusOne,
    Confused,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsCreatePullRequestReviewCommentContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsCreatePullRequestReviewCommentContent::PlusOne => "+1",
            ReactionsCreatePullRequestReviewCommentContent::MinusOne => "-1",
            ReactionsCreatePullRequestReviewCommentContent::Confused => "confused",
            ReactionsCreatePullRequestReviewCommentContent::Eyes => "eyes",
            ReactionsCreatePullRequestReviewCommentContent::Heart => "heart",
            ReactionsCreatePullRequestReviewCommentContent::Hooray => "hooray",
            ReactionsCreatePullRequestReviewCommentContent::Laugh => "laugh",
            ReactionsCreatePullRequestReviewCommentContent::Rocket => "rocket",
            ReactionsCreatePullRequestReviewCommentContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsCreatePullRequestReviewCommentContent {
    fn default() -> ReactionsCreatePullRequestReviewCommentContent {
        ReactionsCreatePullRequestReviewCommentContent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreatePullRequestReviewComment {
    /**
     * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the pull request review comment.
     */
    #[serde()]
    pub content: ReactionsCreatePullRequestReviewCommentContent,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsUpdateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub base: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<PullRequestState>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/**
 * **Required with `comfort-fade` preview**. In a split diff view, the side of the diff that the pull request's changes appear on. Can be `LEFT` or `RIGHT`. Use `LEFT` for deletions that appear in red. Use `RIGHT` for additions that appear in green or unchanged lines that appear in white and are shown for context. For a multi-line comment, side represents whether the last line of the comment range is a deletion or addition. For more information, see "[Diff view options](https://help.github.com/en/articles/about-comparing-branches-in-pull-requests#diff-view-options)" in the GitHub Help documentation.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsCreateReviewCommentRequestSide {
    Left,
    Right,
    Noop,
}

impl std::fmt::Display for PullsCreateReviewCommentRequestSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsCreateReviewCommentRequestSide::Left => "LEFT",
            PullsCreateReviewCommentRequestSide::Right => "RIGHT",
            PullsCreateReviewCommentRequestSide::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullsCreateReviewCommentRequestSide {
    fn default() -> PullsCreateReviewCommentRequestSide {
        PullsCreateReviewCommentRequestSide::Noop
    }
}

/**
 * **Required when using multi-line comments**. To create multi-line comments, you must use the `comfort-fade` preview header. The `start_side` is the starting side of the diff that the comment applies to. Can be `LEFT` or `RIGHT`. To learn more about multi-line comments, see "[Commenting on a pull request](https://help.github.com/en/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)" in the GitHub Help documentation. See `side` in this table for additional context.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsCreateReviewCommentRequestStartSide {
    Left,
    Right,
    Side,
    Noop,
}

impl std::fmt::Display for PullsCreateReviewCommentRequestStartSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsCreateReviewCommentRequestStartSide::Left => "LEFT",
            PullsCreateReviewCommentRequestStartSide::Right => "RIGHT",
            PullsCreateReviewCommentRequestStartSide::Side => "side",
            PullsCreateReviewCommentRequestStartSide::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullsCreateReviewCommentRequestStartSide {
    fn default() -> PullsCreateReviewCommentRequestStartSide {
        PullsCreateReviewCommentRequestStartSide::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsCreateReviewCommentRequest {
    /**
     * The text of the review comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<PullsCreateReviewCommentRequestSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_side: Option<PullsCreateReviewCommentRequestStartSide>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsCreateReplyReviewCommentRequest {
    /**
     * The text of the review comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
}

/**
 * Merge method to use. Possible values are `merge`, `squash` or `rebase`. Default is `merge`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsMergeRequestMethod {
    Merge,
    Rebase,
    Squash,
    Noop,
}

impl std::fmt::Display for PullsMergeRequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsMergeRequestMethod::Merge => "merge",
            PullsMergeRequestMethod::Rebase => "rebase",
            PullsMergeRequestMethod::Squash => "squash",
            PullsMergeRequestMethod::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullsMergeRequestMethod {
    fn default() -> PullsMergeRequestMethod {
        PullsMergeRequestMethod::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsMergeRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_message: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_method: Option<PullsMergeRequestMethod>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsRequestReviewers {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_reviewers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsRemoveRequestedReviewersRequest {
    /**
     * An array of user `login`s that will be removed.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_reviewers: Vec<String>,
}

/**
 * The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request) when you are ready.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsCreateReviewRequestEvent {
    Approve,
    Comment,
    RequestChanges,
    Noop,
}

impl std::fmt::Display for PullsCreateReviewRequestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsCreateReviewRequestEvent::Approve => "APPROVE",
            PullsCreateReviewRequestEvent::Comment => "COMMENT",
            PullsCreateReviewRequestEvent::RequestChanges => "REQUEST_CHANGES",
            PullsCreateReviewRequestEvent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullsCreateReviewRequestEvent {
    fn default() -> PullsCreateReviewRequestEvent {
        PullsCreateReviewRequestEvent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Comments {
    /**
     * Text of the review comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /**
     * The relative path to the file that necessitates a review comment.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub side: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub start_side: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsCreateReviewRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<Comments>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub commit_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<PullsCreateReviewRequestEvent>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsUpdateReviewRequest {
    /**
     * The body text of the pull request review.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsDismissReviewRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub event: String,
    /**
     * The message for the pull request review dismissal
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub message: String,
}

/**
 * The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. When you leave this blank, the API returns _HTTP 422 (Unrecognizable entity)_ and sets the review action state to `PENDING`, which means you will need to re-submit the pull request review using a review action.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PullsSubmitReviewRequestEvent {
    Approve,
    Comment,
    RequestChanges,
    Noop,
}

impl std::fmt::Display for PullsSubmitReviewRequestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PullsSubmitReviewRequestEvent::Approve => "APPROVE",
            PullsSubmitReviewRequestEvent::Comment => "COMMENT",
            PullsSubmitReviewRequestEvent::RequestChanges => "REQUEST_CHANGES",
            PullsSubmitReviewRequestEvent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for PullsSubmitReviewRequestEvent {
    fn default() -> PullsSubmitReviewRequestEvent {
        PullsSubmitReviewRequestEvent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsSubmitReviewRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. When you leave this blank, the API returns _HTTP 422 (Unrecognizable entity)_ and sets the review action state to `PENDING`, which means you will need to re-submit the pull request review using a review action.
     */
    #[serde()]
    pub event: PullsSubmitReviewRequestEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct PullsUpdateBranchRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub expected_head_sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateReleaseRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discussion_category_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    /**
     * The name of the tag.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_commitish: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateReleaseAssetRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub label: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposUpdateReleaseRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub discussion_category_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub tag_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_commitish: String,
}

/**
 * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the release.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReactionsCreateReleaseRequestContent {
    PlusOne,
    Eyes,
    Heart,
    Hooray,
    Laugh,
    Rocket,
    Noop,
}

impl std::fmt::Display for ReactionsCreateReleaseRequestContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReactionsCreateReleaseRequestContent::PlusOne => "+1",
            ReactionsCreateReleaseRequestContent::Eyes => "eyes",
            ReactionsCreateReleaseRequestContent::Heart => "heart",
            ReactionsCreateReleaseRequestContent::Hooray => "hooray",
            ReactionsCreateReleaseRequestContent::Laugh => "laugh",
            ReactionsCreateReleaseRequestContent::Rocket => "rocket",
            ReactionsCreateReleaseRequestContent::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReactionsCreateReleaseRequestContent {
    fn default() -> ReactionsCreateReleaseRequestContent {
        ReactionsCreateReleaseRequestContent::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReactionsCreateReleaseRequest {
    /**
     * The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the release.
     */
    #[serde()]
    pub content: ReactionsCreateReleaseRequestContent,
}

/**
 * Set to `open` or `resolved` to only list secret scanning alerts in a specific state.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SecretScanningListAlertsRepoState {
    Open,
    Resolved,
    Noop,
}

impl std::fmt::Display for SecretScanningListAlertsRepoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SecretScanningListAlertsRepoState::Open => "open",
            SecretScanningListAlertsRepoState::Resolved => "resolved",
            SecretScanningListAlertsRepoState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SecretScanningListAlertsRepoState {
    fn default() -> SecretScanningListAlertsRepoState {
        SecretScanningListAlertsRepoState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct SecretScanningUpdateAlertRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<serde_json::Value>,
    /**
     * Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
     */
    #[serde()]
    pub state: SecretScanningAlertState,
}

/**
 * The state of the status. Can be one of `error`, `failure`, `pending`, or `success`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposCreateCommitStatusRequestState {
    Error,
    Failure,
    Pending,
    Success,
    Noop,
}

impl std::fmt::Display for ReposCreateCommitStatusRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposCreateCommitStatusRequestState::Error => "error",
            ReposCreateCommitStatusRequestState::Failure => "failure",
            ReposCreateCommitStatusRequestState::Pending => "pending",
            ReposCreateCommitStatusRequestState::Success => "success",
            ReposCreateCommitStatusRequestState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposCreateCommitStatusRequestState {
    fn default() -> ReposCreateCommitStatusRequestState {
        ReposCreateCommitStatusRequestState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateCommitStatusRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub context: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The state of the status. Can be one of `error`, `failure`, `pending`, or `success`.
     */
    #[serde()]
    pub state: ReposCreateCommitStatusRequestState,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub target_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActivitySetRepoSubscriptionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposReplaceAllTopicsRequest {
    /**
     * An array of topics to add to the repository. Pass one or more topics to _replace_ the set of existing topics. Send an empty array (`[]`) to clear all topics from the repository. **Note:** Topic `names` cannot contain uppercase letters.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposTransferRequest {
    /**
     * The username or organization name the repository will be transferred to.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub new_owner: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateUsingTemplateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_all_branches: Option<bool>,
    /**
     * The name of the new repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub owner: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ActionsCreateUpdateEnvironmentSecretRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub encrypted_value: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminProvisionInviteGroupRequestMembers {
    /**
     * The SCIM user ID for a user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminProvisionInviteGroupRequest {
    /**
     * The name of the SCIM group. This must match the GitHub organization that the group maps to.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<EnterpriseAdminProvisionInviteGroupRequestMembers>,
    /**
     * The SCIM schema URIs.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
    Add,
    Remove,
    Replace,
    Noop,
}

impl std::fmt::Display for EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Add => "Add",
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Remove => "Remove",
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Replace => "Replace",
            EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
    fn default() -> EnterpriseAdminUpdateAttributeGroupRequestOperationsOp {
        EnterpriseAdminUpdateAttributeGroupRequestOperationsOp::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminUpdateAttributeGroupRequestOperations {
    #[serde()]
    pub op: EnterpriseAdminUpdateAttributeGroupRequestOperationsOp,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminUpdateAttributeGroupRequest {
    /**
     * Array of [SCIM operations](https://tools.ietf.org/html/rfc7644#section-3.5.2).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<EnterpriseAdminUpdateAttributeGroupRequestOperations>,
    /**
     * The SCIM schema URIs.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminProvisionInviteUserRequestName {
    /**
     * The last name of the user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub family_name: String,
    /**
     * The first name of the user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub given_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminProvisionInviteUserRequestEmails {
    /**
     * Whether this email address is the primary address.
     */
    #[serde(default)]
    pub primary: bool,
    /**
     * The type of email address.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    /**
     * The email address.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminProvisionInviteUserRequest {
    /**
     * List of user emails.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<EnterpriseAdminProvisionInviteUserRequestEmails>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ScimUserListEnterpriseResourcesGroups>,
    #[serde()]
    pub name: EnterpriseAdminProvisionInviteUserRequestName,
    /**
     * The SCIM schema URIs.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    /**
     * The username for the user.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct EnterpriseAdminUpdateAttributeUserRequest {
    /**
     * Array of [SCIM operations](https://tools.ietf.org/html/rfc7644#section-3.5.2).
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<Data>,
    /**
     * The SCIM schema URIs.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimProvisionInviteUserRequestEmails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize",
        rename = "type"
    )]
    pub type_: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimProvisionInviteUserRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub display_name: String,
    /**
     * user emails
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<ScimProvisionInviteUserRequestEmails>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    #[serde()]
    pub name: ScimUserName,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
    /**
     * Configured by the admin. Could be an email, login, or username
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Value {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub external_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub family_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub given_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUpdateAttributeUserRequestOperations {
    #[serde()]
    pub op: Op,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ScimUpdateAttributeUserRequest {
    /**
     * Set of operations to be performed
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<ScimUpdateAttributeUserRequestOperations>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemas: Vec<String>,
}

/**
 * Sorts the results of your query. Can only be `indexed`, which indicates how recently a file has been indexed by the GitHub search infrastructure. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchCodeSort {
    Indexed,
    Noop,
}

impl std::fmt::Display for SearchCodeSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SearchCodeSort::Indexed => "indexed",
            SearchCodeSort::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SearchCodeSort {
    fn default() -> SearchCodeSort {
        SearchCodeSort::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetSearchCodeResponse {
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CodeSearchResultItem>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by `author-date` or `committer-date`. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchCommitsSort {
    AuthorDate,
    CommitterDate,
    Noop,
}

impl std::fmt::Display for SearchCommitsSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SearchCommitsSort::AuthorDate => "author-date",
            SearchCommitsSort::CommitterDate => "committer-date",
            SearchCommitsSort::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SearchCommitsSort {
    fn default() -> SearchCommitsSort {
        SearchCommitsSort::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetSearchCommitsResponse {
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CommitSearchResultItem>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by the number of `comments`, `reactions`, `reactions-+1`, `reactions--1`, `reactions-smile`, `reactions-thinking_face`, `reactions-heart`, `reactions-tada`, or `interactions`. You can also sort results by how recently the items were `created` or `updated`, Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchIssuesPullRequestsSort {
    Comments,
    Created,
    Interactions,
    Reactions,
    ReactionsPlusOne,
    ReactionsMinusOne,
    ReactionsHeart,
    ReactionsSmile,
    ReactionsTada,
    ReactionsThinkingFace,
    Updated,
    Noop,
}

impl std::fmt::Display for SearchIssuesPullRequestsSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SearchIssuesPullRequestsSort::Comments => "comments",
            SearchIssuesPullRequestsSort::Created => "created",
            SearchIssuesPullRequestsSort::Interactions => "interactions",
            SearchIssuesPullRequestsSort::Reactions => "reactions",
            SearchIssuesPullRequestsSort::ReactionsPlusOne => "reactions-+1",
            SearchIssuesPullRequestsSort::ReactionsMinusOne => "reactions--1",
            SearchIssuesPullRequestsSort::ReactionsHeart => "reactions-heart",
            SearchIssuesPullRequestsSort::ReactionsSmile => "reactions-smile",
            SearchIssuesPullRequestsSort::ReactionsTada => "reactions-tada",
            SearchIssuesPullRequestsSort::ReactionsThinkingFace => "reactions-thinking_face",
            SearchIssuesPullRequestsSort::Updated => "updated",
            SearchIssuesPullRequestsSort::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SearchIssuesPullRequestsSort {
    fn default() -> SearchIssuesPullRequestsSort {
        SearchIssuesPullRequestsSort::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetSearchIssuesPullRequestsResponse {
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IssueSearchResultItem>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by when the label was `created` or `updated`. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchLabelsSort {
    Created,
    Updated,
    Noop,
}

impl std::fmt::Display for SearchLabelsSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SearchLabelsSort::Created => "created",
            SearchLabelsSort::Updated => "updated",
            SearchLabelsSort::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SearchLabelsSort {
    fn default() -> SearchLabelsSort {
        SearchLabelsSort::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetSearchLabelsResponse {
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LabelSearchResultItem>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by number of `stars`, `forks`, or `help-wanted-issues` or how recently the items were `updated`. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchReposSort {
    Forks,
    HelpWantedIssues,
    Stars,
    Updated,
    Noop,
}

impl std::fmt::Display for SearchReposSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SearchReposSort::Forks => "forks",
            SearchReposSort::HelpWantedIssues => "help-wanted-issues",
            SearchReposSort::Stars => "stars",
            SearchReposSort::Updated => "updated",
            SearchReposSort::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SearchReposSort {
    fn default() -> SearchReposSort {
        SearchReposSort::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetSearchReposResponse {
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RepoSearchResultItem>,
    #[serde(default)]
    pub total_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetSearchTopicsResponse {
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TopicSearchResultItem>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * Sorts the results of your query by number of `followers` or `repositories`, or when the person `joined` GitHub. Default: [best match](https://docs.github.com/rest/reference/search#ranking-search-results)
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SearchUsersSort {
    Followers,
    Joined,
    Repositories,
    Noop,
}

impl std::fmt::Display for SearchUsersSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SearchUsersSort::Followers => "followers",
            SearchUsersSort::Joined => "joined",
            SearchUsersSort::Repositories => "repositories",
            SearchUsersSort::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SearchUsersSort {
    fn default() -> SearchUsersSort {
        SearchUsersSort::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetSearchUsersResponse {
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UserSearchResultItem>,
    #[serde(default)]
    pub total_count: i64,
}

/**
 * The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. The options are:  
 *   **For a non-nested team:**  
 *   \* `secret` - only visible to organization owners and members of this team.  
 *   \* `closed` - visible to all members of this organization.  
 *   **For a parent or child team:**  
 *   \* `closed` - visible to all members of this organization.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamsUpdateLegacyRequestPrivacy {
    Closed,
    Secret,
    Noop,
}

impl std::fmt::Display for TeamsUpdateLegacyRequestPrivacy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamsUpdateLegacyRequestPrivacy::Closed => "closed",
            TeamsUpdateLegacyRequestPrivacy::Secret => "secret",
            TeamsUpdateLegacyRequestPrivacy::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for TeamsUpdateLegacyRequestPrivacy {
    fn default() -> TeamsUpdateLegacyRequestPrivacy {
        TeamsUpdateLegacyRequestPrivacy::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsUpdateLegacyRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * The name of the team.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_team_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<TeamsUpdateLegacyRequestPrivacy>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateProjectPermissionsLegacyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<TeamsAddUpdateProjectPermissionsInOrgRequestPermission>,
}

/**
 * The permission to grant the team on this repository. Can be one of:  
 *   \* `pull` - team members can pull, but not push to or administer this repository.  
 *   \* `push` - team members can pull and push, but not administer this repository.  
 *   \* `admin` - team members can pull, push and administer this repository.  
 *     
 *   If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamsAddUpdateRepoPermissionsLegacyRequestPermission {
    Admin,
    Pull,
    Push,
    Noop,
}

impl std::fmt::Display for TeamsAddUpdateRepoPermissionsLegacyRequestPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TeamsAddUpdateRepoPermissionsLegacyRequestPermission::Admin => "admin",
            TeamsAddUpdateRepoPermissionsLegacyRequestPermission::Pull => "pull",
            TeamsAddUpdateRepoPermissionsLegacyRequestPermission::Push => "push",
            TeamsAddUpdateRepoPermissionsLegacyRequestPermission::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for TeamsAddUpdateRepoPermissionsLegacyRequestPermission {
    fn default() -> TeamsAddUpdateRepoPermissionsLegacyRequestPermission {
        TeamsAddUpdateRepoPermissionsLegacyRequestPermission::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsAddUpdateRepoPermissionsLegacyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<TeamsAddUpdateRepoPermissionsLegacyRequestPermission>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateUpdateIdpGroupConnectionsLegacyRequestGroups {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    /**
     * Description of the IdP group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_description: String,
    /**
     * ID of the IdP group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_id: String,
    /**
     * Name of the IdP group.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub group_name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TeamsCreateUpdateIdpGroupConnectionsLegacyRequest {
    /**
     * The IdP groups you want to connect to a GitHub team. When updating, the new `groups` object will replace the original one. You must include any existing groups that you don't want to remove.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<TeamsCreateUpdateIdpGroupConnectionsLegacyRequestGroups>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub synced_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersUpdateAuthenticatedRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub bio: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub blog: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub company: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hireable: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub location: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub twitter_username: String,
}

/**
 * Denotes whether an email is publicly visible.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UsersSetPrimaryEmailVisibilityAuthenticatedRequest {
    Private,
    Public,
    Noop,
}

impl std::fmt::Display for UsersSetPrimaryEmailVisibilityAuthenticatedRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UsersSetPrimaryEmailVisibilityAuthenticatedRequest::Private => "private",
            UsersSetPrimaryEmailVisibilityAuthenticatedRequest::Public => "public",
            UsersSetPrimaryEmailVisibilityAuthenticatedRequest::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for UsersSetPrimaryEmailVisibilityAuthenticatedRequest {
    fn default() -> UsersSetPrimaryEmailVisibilityAuthenticatedRequest {
        UsersSetPrimaryEmailVisibilityAuthenticatedRequest::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersSetPrimaryEmailVisibilityAuthenticatedRequestData {
    /**
     * Denotes whether an email is publicly visible.
     */
    #[serde()]
    pub visibility: UsersSetPrimaryEmailVisibilityAuthenticatedRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersAddEmailAuthenticatedRequest {
    /**
     * Adds one or more email addresses to your GitHub account. Must contain at least one email address. **Note:** Alternatively, you can pass a single email address or an `array` of emails addresses directly, but we recommend that you pass an object using the `emails` key.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
}

/// Deletes one or more email addresses from your GitHub account. Must contain at least one email address. **Note:** Alternatively, you can pass a single email address or an `array` of emails addresses directly, but we recommend that you pass an object using the `emails` key.
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersDeleteEmailAuthenticatedRequest {
    /**
     * Email addresses associated with the GitHub user account.
     */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersCreateGpgKeyAuthenticatedRequest {
    /**
     * A GPG key in ASCII-armored format.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub armored_public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct UsersCreatePublicSshKeyAuthenticatedRequest {
    /**
     * The public SSH key to add to your GitHub account.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub key: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub title: String,
}

/**
 * Indicates the state of the memberships to return. Can be either `active` or `pending`. If not specified, the API returns both active and pending memberships.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrgsListMembershipsState {
    Active,
    Pending,
    Noop,
}

impl std::fmt::Display for OrgsListMembershipsState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrgsListMembershipsState::Active => "active",
            OrgsListMembershipsState::Pending => "pending",
            OrgsListMembershipsState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrgsListMembershipsState {
    fn default() -> OrgsListMembershipsState {
        OrgsListMembershipsState::Noop
    }
}

/**
 * The state that the membership should be in. Only `"active"` will be accepted.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OrgsUpdateMembershipRequestState {
    Active,
    Noop,
}

impl std::fmt::Display for OrgsUpdateMembershipRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OrgsUpdateMembershipRequestState::Active => "active",
            OrgsUpdateMembershipRequestState::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for OrgsUpdateMembershipRequestState {
    fn default() -> OrgsUpdateMembershipRequestState {
        OrgsUpdateMembershipRequestState::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct OrgsUpdateMembershipRequest {
    /**
     * The state that the membership should be in. Only `"active"` will be accepted.
     */
    #[serde()]
    pub state: OrgsUpdateMembershipRequestState,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct MigrationsStartRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<Exclude>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_attachments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ProjectsCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub body: String,
    /**
     * Name of the project
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
}

/**
 * Can be one of `all`, `public`, or `private`. Note: For GitHub AE, can be one of `all`, `internal`, or `private`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListVisibility {
    All,
    Private,
    Public,
}

impl std::fmt::Display for ReposListVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListVisibility::All => "all",
            ReposListVisibility::Private => "private",
            ReposListVisibility::Public => "public",
        }
        .fmt(f)
    }
}

impl Default for ReposListVisibility {
    fn default() -> ReposListVisibility {
        ReposListVisibility::All
    }
}

/**
 * Can be one of `all`, `owner`, `public`, `private`, `member`. Note: For GitHub AE, can be one of `all`, `owner`, `internal`, `private`, `member`. Default: `all`  
 *     
 *   Will cause a `422` error if used in the same request as **visibility** or **affiliation**. Will cause a `422` error if used in the same request as **visibility** or **affiliation**.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListType {
    All,
    Member,
    Owner,
    Private,
    Public,
}

impl std::fmt::Display for ReposListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListType::All => "all",
            ReposListType::Member => "member",
            ReposListType::Owner => "owner",
            ReposListType::Private => "private",
            ReposListType::Public => "public",
        }
        .fmt(f)
    }
}

impl Default for ReposListType {
    fn default() -> ReposListType {
        ReposListType::All
    }
}

/**
 * Can be one of `asc` or `desc`. Default: `asc` when using `full_name`, otherwise `desc`
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListDirection {
    Asc,
    Desc,
    Noop,
}

impl std::fmt::Display for ReposListDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListDirection::Asc => "asc",
            ReposListDirection::Desc => "desc",
            ReposListDirection::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for ReposListDirection {
    fn default() -> ReposListDirection {
        ReposListDirection::Noop
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ReposCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_init: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub description: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub gitignore_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub homepage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub license_template: String,
    /**
     * The name of the repository.
     */
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
}

/**
 * Identifies which additional information you'd like to receive about the person's hovercard. Can be `organization`, `repository`, `issue`, `pull_request`. **Required** when using `subject_id`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubjectType {
    Issue,
    Organization,
    PullRequest,
    Repository,
    Noop,
}

impl std::fmt::Display for SubjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SubjectType::Issue => "issue",
            SubjectType::Organization => "organization",
            SubjectType::PullRequest => "pull_request",
            SubjectType::Repository => "repository",
            SubjectType::Noop => "",
        }
        .fmt(f)
    }
}

impl Default for SubjectType {
    fn default() -> SubjectType {
        SubjectType::Noop
    }
}

/**
 * Can be one of `all`, `owner`, `member`.
 */
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReposListUserType {
    All,
    Member,
    Owner,
}

impl std::fmt::Display for ReposListUserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReposListUserType::All => "all",
            ReposListUserType::Member => "member",
            ReposListUserType::Owner => "owner",
        }
        .fmt(f)
    }
}

impl Default for ReposListUserType {
    fn default() -> ReposListUserType {
        ReposListUserType::Owner
    }
}
