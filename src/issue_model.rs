use chrono::{DateTime, NaiveDate, Utc};
/// This module replicates the fields in a Jira issue as strongly typed structs.
/// Any extra fields that come from a custom Jira configuration are captured
/// in the `extra` hash map in the parent struct.
use serde::Deserialize;
use serde_json::Value;

/// The response from Jira to a JQL query,
/// which includes the list of requested issues and additional metadata.
#[derive(Clone, Debug, Deserialize)]
pub struct JqlResults {
    pub issues: Vec<Issue>,
    #[serde(flatten)]
    pub extra: Value,
}

/// A single Jira issue with all its fields.
#[derive(Clone, Debug, Deserialize)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub expand: String,
    pub fields: Fields,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A container for most fields of a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Fields {
    #[serde(rename = "lastViewed")]
    pub last_viewed: Option<DateTime<Utc>>,
    pub labels: Vec<String>,
    pub versions: Vec<Version>,
    pub assignee: Option<User>,
    pub description: Option<String>,
    pub duedate: Option<DateTime<Utc>>,
    #[serde(rename = "fixVersions")]
    pub fix_versions: Vec<Version>,
    pub reporter: User,
    pub status: Status,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub issuetype: IssueType,
    pub timeestimate: Option<i32>,
    pub aggregatetimeestimate: Option<i32>,
    pub timeoriginalestimate: Option<i32>,
    pub timespent: Option<i32>,
    pub aggregatetimespent: Option<i32>,
    pub aggregatetimeoriginalestimate: Option<i32>,
    pub progress: Progress,
    pub aggregateprogress: Progress,
    pub workratio: i32,
    pub summary: String,
    pub creator: User,
    pub project: Project,
    pub priority: Priority,
    pub components: Vec<Component>,
    pub watches: Watches,
    pub archiveddate: Option<DateTime<Utc>>,
    pub archivedby: Option<DateTime<Utc>>,
    pub resolution: Option<Resolution>,
    pub resolutiondate: Option<DateTime<Utc>>,
    pub comment: Option<Comments>,
    pub issuelinks: Vec<IssueLink>,
    pub votes: Votes,
    pub parent: Option<CondensedIssue>,
    pub subtasks: Vec<CondensedIssue>,
    #[serde(flatten)]
    pub extra: Value,
}

/// The representation of a Jira user account.
#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub active: bool,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    pub key: String,
    pub name: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The representation of a Jira product version.
#[derive(Clone, Debug, Deserialize)]
pub struct Version {
    pub id: String,
    pub description: Option<String>,
    pub name: String,
    pub archived: bool,
    pub released: bool,
    /// Jira stores `releaseDate` only as `YYYY-MM-DD`, so it can't deserialize to full `DateTime`.
    #[serde(rename = "releaseDate")]
    pub release_date: Option<NaiveDate>,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The Jira issue status.
#[derive(Clone, Debug, Deserialize)]
pub struct Status {
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "statusCategory")]
    pub status_category: StatusCategory,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The category of a Jira issue status.
#[derive(Clone, Debug, Deserialize)]
pub struct StatusCategory {
    #[serde(rename = "colorName")]
    pub color_name: String,
    pub id: i32,
    pub key: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The resolution of a Jira issue when it's closed.
#[derive(Clone, Debug, Deserialize)]
pub struct Resolution {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The type of a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct IssueType {
    #[serde(rename = "avatarId")]
    pub avatar_id: i32,
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    pub subtask: bool,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A project namespace that groups Jira issues.
#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "projectTypeKey")]
    pub project_type_key: String,
    #[serde(rename = "projectCategory")]
    pub project_category: ProjectCategory,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The category of a Jira project.
#[derive(Clone, Debug, Deserialize)]
pub struct ProjectCategory {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The priority of a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Priority {
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The component of a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Component {
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// Users watching a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Watches {
    #[serde(rename = "isWatching")]
    pub is_watching: bool,
    #[serde(rename = "watchCount")]
    pub watch_count: i32,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The progress of a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Progress {
    pub progress: i32,
    pub total: i32,
    #[serde(flatten)]
    pub extra: Value,
}

/// A comment below a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Comment {
    pub author: User,
    pub body: String,
    pub created: DateTime<Utc>,
    pub id: String,
    #[serde(rename = "updateAuthor")]
    pub update_author: User,
    pub updated: DateTime<Utc>,
    pub visibility: Option<Visibility>,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A container for all comments below a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Comments {
    pub comments: Vec<Comment>,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    #[serde(rename = "startAt")]
    pub start_at: i32,
    pub total: i32,
    #[serde(flatten)]
    pub extra: Value,
}

/// A link from one Jira issue to another.
#[derive(Clone, Debug, Deserialize)]
pub struct IssueLink {
    pub id: String,
    #[serde(rename = "outwardIssue")]
    pub outward_issue: Option<LinkedIssue>,
    #[serde(rename = "inwardIssue")]
    pub inward_issue: Option<LinkedIssue>,
    #[serde(rename = "type")]
    pub link_type: IssueLinkType,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A Jira issue linked from another one.
#[derive(Clone, Debug, Deserialize)]
pub struct LinkedIssue {
    pub id: String,
    pub key: String,
    pub fields: LinkedIssueFields,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The reduced fields of a linked Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct LinkedIssueFields {
    pub issuetype: IssueType,
    pub priority: Option<Priority>,
    pub status: Status,
    pub summary: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The direction of a link to a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct IssueLinkType {
    pub id: String,
    pub inward: String,
    pub name: String,
    pub outward: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The votes for a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Votes {
    #[serde(rename = "hasVoted")]
    pub has_voted: bool,
    pub votes: i32,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A Jira avatar in several different sizes:
///
/// * `xsmall` = 16x16 px
/// * `small` = 24x24 px
/// * `medium` = 48x48 px
/// * `full` = maximum
#[derive(Clone, Debug, Deserialize)]
pub struct AvatarUrls {
    #[serde(rename = "16x16")]
    pub xsmall: String,
    #[serde(rename = "24x24")]
    pub small: String,
    #[serde(rename = "32x32")]
    pub medium: String,
    #[serde(rename = "48x48")]
    pub full: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A minimal, reduced representation of a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct CondensedIssue {
    pub fields: CondensedFields,
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// A minimal, reduced listing of the fields of a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct CondensedFields {
    pub issuetype: IssueType,
    pub priority: Priority,
    pub status: Status,
    pub summary: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The visibility of a Jira issue.
#[derive(Clone, Debug, Deserialize)]
pub struct Visibility {
    pub r#type: String,
    pub value: String,
    #[serde(flatten)]
    pub extra: Value,
}
