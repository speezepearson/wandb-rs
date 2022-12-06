use graphql_client::{GraphQLQuery, Response};

use crate::core::*;

mod queries;

pub enum EntityKey {
    Name(EntityName),
}
pub struct Entity;

impl ProjectKey {
    pub fn from_name(entity_name: impl Into<EntityName>, project_name: impl Into<ProjectName>) -> Self {
        Self::Name(entity_name.into(), project_name.into())
    }
}
#[derive(Debug)]
pub enum ProjectKeyParseErr {
    InvalidFormat,
}
impl std::str::FromStr for ProjectKey {
    type Err = ProjectKeyParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        regex::Regex::new(r"^(?P<entity_name>[\w.-]+)/(?P<project_name>[\w.-]+)$")
            .unwrap()
            .captures(s)
            .ok_or(ProjectKeyParseErr::InvalidFormat)
            .map(|captures| Self::Name(
                captures.name("entity_name").unwrap().as_str().to_owned(),
                captures.name("project_name").unwrap().as_str().to_owned(),
            ))
    }
}
pub struct Project;

pub type RunName = String;
pub enum RunKey {
    Name(ProjectKey, RunName),
}
impl RunKey {
    pub fn from_name(entity_name: impl Into<EntityName>, project_key: impl Into<ProjectName>, run_name: impl Into<RunName>) -> Self {
        Self::Name(ProjectKey::from_name(entity_name, project_key), run_name.into())
    }
}
#[derive(Debug)]
pub enum RunKeyParseErr {
    InvalidFormat,
}
impl std::str::FromStr for RunKey {
    type Err = RunKeyParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        regex::Regex::new(r"^(?P<entity_name>[\w.-]+)/(?P<project_name>[\w.-]+)/(?P<run_name>[\w.-]+)$")
            .unwrap()
            .captures(s)
            .ok_or(RunKeyParseErr::InvalidFormat)
            .map(|captures| Self::Name(
                ProjectKey::Name(
                    captures.name("entity_name").unwrap().as_str().to_owned(),
                    captures.name("project_name").unwrap().as_str().to_owned(),
                ),
                captures.name("run_name").unwrap().as_str().to_owned(),
            ))
    }
}
pub struct Run;

pub struct Api {
    client: reqwest::Client,
    http_endpoint: url::Url,
    api_key: Option<String>,
}

fn get_api_key() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let netrc_path = std::path::Path::new(&home).join(".netrc");
    let netrc_file_contents = std::fs::read_to_string(netrc_path).ok()?;

    // search for the three-line regex
    let re = regex::Regex::new(r#"machine api\.wandb\.ai
 +login user
 +password ([a-zA-Z0-9]+)"#).unwrap();
    let captures = re.captures(&netrc_file_contents)?;

    Some(captures.get(1).unwrap().as_str().to_string())
}

impl Default for Api {
    fn default() -> Self {
        // Self::new("http://localhost:8333".parse().unwrap())
        Self::new("https://api.wandb.ai/graphql".parse().unwrap())
    }
}

pub type ApiResult<R> = Result<R, Box<dyn std::error::Error>>;

impl Api {

    fn new(http_endpoint: url::Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            http_endpoint,
            api_key: get_api_key(),
        }
    }

    async fn send<T: serde::Serialize, R: serde::de::DeserializeOwned>(&self, query: T) -> ApiResult<R> {
        let mut req = self.client.post(self.http_endpoint.as_str());
        if let Some(api_key) = &self.api_key {
            req = req.basic_auth("Bearer", Some(api_key.as_str()));
        }
        let resp = req.json(&query).send().await?;
        let resp_body: Response<R> = resp.json().await?;
        resp_body.data.ok_or_else(|| "no data".into())
    }

    pub async fn entity(&self, key: &EntityKey) -> Entity {
        match key {
            EntityKey::Name(..) => unimplemented!(),
        }
    }

    pub async fn project(&self, key: &ProjectKey) -> ApiResult<queries::projects::project_by_name::ResponseData> {
        match key {
            ProjectKey::Name(entity_name, project_name) => {
                self.send(queries::projects::ProjectByName::build_query(queries::projects::project_by_name::Variables{
                    entity_name: entity_name.to_string(),
                    project_name: project_name.to_string(),
                })).await
            }
        }
    }

    pub async fn run(&self, key: &RunKey) -> Run {
        match key {
            RunKey::Name(..) => unimplemented!(),
        }
    }

    pub async fn viewer(&self) -> ApiResult<queries::viewer::viewer::ResponseData> {
        self.send(queries::viewer::Viewer::build_query( queries::viewer::viewer::Variables)).await
    }

    pub async fn upsert_bucket(&self, entity_name: Option<String>, project_name: Option<String>) -> ApiResult<queries::upsert_bucket::upsert_bucket::ResponseData> {
        self.send(queries::upsert_bucket::UpsertBucket::build_query(queries::upsert_bucket::upsert_bucket::Variables{
            entity: entity_name,
            project: project_name,
        })).await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn project_query_smoke() {
        let server = httpmock::MockServer::start();
        let url: url::Url = server.url("/graphql").parse().unwrap();

        // Create a mock on the server.
        let hello_mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/graphql");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{ "data": {"project": { "id": "some id", "name": "GPT-123" }} }"#);
        });

        // Send an HTTP request to the mock server. This simulates your code.
        let response = Api::new(url).project(&ProjectKey::from_name("foo", "bar")).await.unwrap();

        // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
        hello_mock.assert();
        // Ensure the mock server did respond as specified.
        assert_eq!(response.project, Some(queries::projects::project_by_name::ProjectByNameProject{
            id: "some id".into(),
            name: "GPT-123".into(),
        }));
    }

    #[tokio::test]
    async fn viewer_query_smoke() {
        let server = httpmock::MockServer::start();
        let url: url::Url = server.url("/graphql").parse().unwrap();

        // Create a mock on the server.
        let hello_mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/graphql");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{ "data": {"viewer": { "id": "some id", "name": "Testy McTestface" }} }"#);
        });

        // Send an HTTP request to the mock server. This simulates your code.
        let response = Api::new(url).viewer().await.unwrap();

        // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
        hello_mock.assert();
        // Ensure the mock server did respond as specified.
        assert_eq!(response.viewer, Some(queries::viewer::viewer::ViewerViewer{
            id: "some id".into(),
            name: "Testy McTestface".into(),
        }));
    }

}
