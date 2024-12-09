use crate::state::types::{Author, Review, UserId};
use octocrab::models::Author as OctocrabAuthor;
use octocrab::models::UserId as OctocrabUserId;
use octocrab::{models, Octocrab};
use std::error::Error;
use std::sync::Arc;

pub async fn get_all_relevant_prs(
    username: &str,
    token: &str,
) -> Result<Vec<Review>, octocrab::Error> {
    let octocrab = Octocrab::builder()
        .personal_token(String::from(token))
        .build()?;

    let search_query = format!("type:pr state:open review-requested:{}", username);
    println!("GitHub API Search Query: {}", search_query);

    let search_results = match octocrab
        .search()
        .issues_and_pull_requests(&search_query)
        .per_page(100)
        .sort("updated")
        .order("desc")
        .send()
        .await
    {
        Ok(results) => results,
        Err(err) => {
            println!("GitHub API Error: {:?}", err);
            if let Some(source) = err.source() {
                println!("Error source: {:?}", source);
            }
            return Err(err);
        }
    };

    println!(
        "Search Results: {}",
        serde_json::to_string_pretty(&search_results.items).unwrap()
    );
    println!("Number of results: {}", search_results.items.len());

    Ok(search_results
        .items
        .into_iter()
        .map(|issue| Review {
            repository: issue.repository_url.to_string(),
            author: issue.user.login.clone(),
            author_object: Author::from(issue.user),
            created_at: issue.created_at.to_string(),
            number: issue.number,
            url: issue.html_url.to_string(),
            title: issue.title,
            closed: issue.state == models::IssueState::Closed,
            is_draft: false,
            review_decision: String::from(""),
            total_comments_count: issue.comments,
        })
        .collect())
}

impl From<OctocrabAuthor> for Author {
    fn from(author: OctocrabAuthor) -> Self {
        Author {
            login: author.login,
            id: UserId::from(author.id),
            node_id: author.node_id,
            avatar_url: author.avatar_url.to_string(),
            gravatar_id: author.gravatar_id,
            url: author.url.to_string(),
            html_url: author.html_url.to_string(),
            followers_url: author.followers_url.to_string(),
            following_url: author.following_url.to_string(),
            gists_url: author.gists_url.to_string(),
            starred_url: author.starred_url.to_string(),
            subscriptions_url: author.subscriptions_url.to_string(),
            organizations_url: author.organizations_url.to_string(),
            repos_url: author.repos_url.to_string(),
            events_url: author.events_url.to_string(),
            received_events_url: author.received_events_url.to_string(),
            r#type: author.r#type,
            site_admin: author.site_admin,
            patch_url: author.patch_url,
            email: author.email,
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

    println!("Current user: {:?}", current_user);

    Ok(Author::from(current_user))
}

pub async fn login_user(token: &str, hostname: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create the Octocrab instance with the provided hostname as base URI
    let octocrab = Octocrab::builder()
        .personal_token(String::from(token))
        .base_uri(hostname.parse::<http::Uri>()?)?
        .build()?;

    // Get authenticated user
    let current_user = octocrab.current().user().await?;

    println!("Current user: {:?}", current_user);

    Ok(())
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
                author: issue.user.login.clone(),
                author_object: Author::from(issue.user),
                created_at: issue.created_at.to_string(),
                number: issue.number,
                url: issue.html_url.to_string(),
                title: issue.title,
                closed: issue.state == models::IssueState::Closed,
                is_draft: false,
                review_decision: String::from(""),
                total_comments_count: issue.comments,
            })
            .collect())
    }

    pub async fn get_user_info(&self) -> Result<Author, Box<dyn std::error::Error>> {
        let current_user = self.client().current().user().await?;
        println!("Current user: {:?}", current_user);
        Ok(Author::from(current_user))
    }
}
