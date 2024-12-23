//! Run this to update `github_schema.graphql`:
//!
//! ```sh
//! curl -L https://docs.github.com/public/fpt/schema.docs.graphql -o src-tauri/src/state/github/github_schema.graphql
//! ```

use crate::state::types::{Author, Label, Labels, PullRequest, Review, StatusCheckRollup, UserId};
use graphql_client::GraphQLQuery;
use log::info;
use octocrab::models::Author as OctocrabAuthor;
use octocrab::models::UserId as OctocrabUserId;
use octocrab::{models, Octocrab};
use std::sync::Arc;

type URI = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/state/github/github_schema.graphql",
    query_path = "src/state/github/pull_requests.graphql",
    response_derives = "Debug, Clone, Serialize",
    variables_derives = "Clone, Debug"
)]
pub struct PullRequests;

impl From<pull_requests::PullRequestsSearchNodesOnPullRequest> for PullRequest {
    fn from(node: pull_requests::PullRequestsSearchNodesOnPullRequest) -> Self {
        PullRequest {
            title: node.title,
            url: node.url,
            number: node.number,
            repository: node.repository.name_with_owner,
            author: Author {
                login: node
                    .author
                    .as_ref()
                    .map(|a| a.login.clone())
                    .unwrap_or_default(),
                avatar_url: node
                    .author
                    .as_ref()
                    .map(|a| a.avatar_url.clone())
                    .unwrap_or_default(),
                url: node
                    .author
                    .as_ref()
                    .map(|a| a.url.clone())
                    .unwrap_or_default(),
            },
            created_at: node.created_at,
            closed: node.closed,
            is_draft: node.is_draft,
            review_decision: node
                .review_decision
                .map(|d| match d {
                    pull_requests::PullRequestReviewDecision::REVIEW_REQUIRED => {
                        String::from("REVIEW_REQUIRED")
                    }
                    pull_requests::PullRequestReviewDecision::APPROVED => String::from("APPROVED"),
                    pull_requests::PullRequestReviewDecision::CHANGES_REQUESTED => {
                        String::from("CHANGES_REQUESTED")
                    }
                    // Add any other possible values from the enum
                    _ => String::from(""),
                })
                .unwrap_or_else(|| String::from("")),
            total_comments_count: node.comments.total_count,
            is_read_by_viewer: matches!(
                node.viewer_subscription,
                Some(pull_requests::SubscriptionState::SUBSCRIBED)
            ),
            labels: Labels(
                node.labels
                    .and_then(|l| l.nodes)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten() // Handle the Some/None within the vector
                    .map(|label| Label {
                        name: label.name,
                        color: label.color,
                    })
                    .collect(),
            ),
            status_check_rollup: StatusCheckRollup {
                state: node
                    .commits
                    .nodes
                    .and_then(|nodes| nodes.first().cloned())
                    .flatten()
                    .and_then(|commit_node| commit_node.commit.status_check_rollup)
                    .map(|rollup| match rollup.state {
                        pull_requests::StatusState::PENDING => String::from("PENDING"),
                        pull_requests::StatusState::SUCCESS => String::from("SUCCESS"),
                        pull_requests::StatusState::FAILURE => String::from("FAILURE"),
                        // Add other states as needed
                        _ => String::from(""),
                    })
                    .unwrap_or_default(),
            },
        }
    }
}

impl From<OctocrabAuthor> for Author {
    fn from(author: OctocrabAuthor) -> Self {
        Author {
            login: author.login,
            avatar_url: author.avatar_url.to_string(),
            url: author.url.to_string(),
        }
    }
}

impl From<OctocrabUserId> for UserId {
    fn from(id: OctocrabUserId) -> Self {
        UserId(id.0)
    }
}

pub async fn get_user_info(token: &str) -> Result<Author, Box<dyn std::error::Error>> {
    // Create an instance of Octocrab
    let octocrab = Octocrab::builder()
        .personal_token(String::from(token))
        .build()?;

    // Get authenticated user (if using authentication)
    let current_user = octocrab.current().user().await?;

    info!("Current user: {:?}", current_user);

    Ok(Author::from(current_user))
}

// Client wrapper struct
#[derive(Clone)]
pub struct GitHubClient {
    inner: Arc<Octocrab>,
}

impl GitHubClient {
    // Constructor
    pub fn new(token: &str) -> Result<Self, octocrab::Error> {
        let client = Octocrab::builder()
            .personal_token(String::from(token))
            .build()?;

        Ok(Self {
            inner: Arc::new(client),
        })
    }

    // Constructor for enterprise GitHub
    pub fn new_enterprise(token: &str, hostname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Octocrab::builder()
            .personal_token(String::from(token))
            .base_uri(hostname.parse::<http::Uri>()?)?
            .build()?;

        Ok(Self {
            inner: Arc::new(client),
        })
    }

    pub fn create_client(
        token: &str,
        hostname: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match hostname {
            Some(host) => Self::new_enterprise(&token, &host),
            None => Self::new(token).map_err(|e| e.into()),
        }
    }

    // Method to get the inner client
    pub fn client(&self) -> &Octocrab {
        &self.inner
    }

    // Implement your API methods here
    pub async fn get_all_relevant_prs(
        &self,
        username: &str,
    ) -> Result<Vec<Review>, octocrab::Error> {
        let search_query = format!("type:pr state:open review-requested:{}", username);

        let search_results = self
            .client()
            .search()
            .issues_and_pull_requests(&search_query)
            .per_page(100)
            .sort("updated")
            .order("desc")
            .send()
            .await?;

        Ok(search_results
            .items
            .into_iter()
            .map(|issue| Review {
                repository: issue.repository_url.to_string(),
                author: Author::from(issue.user),
                created_at: issue.created_at.to_string(),
                number: issue.number,
                url: issue.html_url.to_string(),
                title: issue.title,
                closed: issue.state == models::IssueState::Closed,
                is_draft: false,
                review_decision: String::from(""),
                total_comments_count: issue.comments,
                is_read_by_viewer: false,
                labels: Labels(
                    issue
                        .labels
                        .into_iter()
                        .map(|label| Label {
                            name: label.name,
                            color: label.color,
                        })
                        .collect(),
                ),
                status_check_rollup: StatusCheckRollup {
                    state: String::from(""),
                },
            })
            .collect())
    }

    pub async fn get_user_info(&self) -> Result<Author, Box<dyn std::error::Error>> {
        let current_user = self.client().current().user().await?;
        info!("Current user: {:?}", current_user);
        Ok(Author::from(current_user))
    }

    pub async fn get_all_relevant_prs_2(
        &self,
        username: &str,
    ) -> Result<Vec<PullRequest>, Box<dyn std::error::Error>> {
        let variables = pull_requests::Variables {
            query: format!("type:pr state:open review-requested:{}", username),
            per_page: 100,
        };

        let response: serde_json::Value = self
            .client()
            .graphql(&PullRequests::build_query(variables.clone()))
            .await?;

        // Deserialize the response into our expected type
        let data: graphql_client::Response<pull_requests::ResponseData> =
            serde_json::from_value(response)?;

        // Access the data field first, then access search
        let reviews = data
            .data
            .ok_or_else(|| Box::<dyn std::error::Error>::from("No data received"))?
            .search
            .nodes
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter_map(|node| {
                if let pull_requests::PullRequestsSearchNodes::PullRequest(pr) = node {
                    Some(PullRequest::from(pr))
                } else {
                    None
                }
            })
            .collect();

        Ok(reviews)
    }
}
