use std::collections::HashMap;

pub mod api;
pub mod core;

pub struct Run<T> {
    entity_name: String,
    project_name: String,
    run_name: String,

    _phantom: std::marker::PhantomData<T>,
}

impl<T> Run<T> {
    pub fn url(&self) -> url::Url {
        let mut res = url::Url::parse("https://wandb.ai/").unwrap();
        res.set_path(&format!("{}/{}/runs/{}", self.entity_name, self.project_name, self.run_name));
        res
    }
}

pub struct ActiveRunMarker;
pub struct FinishedRunMarker;
pub type ActiveRun = Run<ActiveRunMarker>;
pub type FinishedRun = Run<FinishedRunMarker>;

pub enum LogValue {
    String(String),
    Number(f64),
    Bool(bool),
}

impl From<String> for LogValue {
    fn from(s: String) -> Self {
        LogValue::String(s)
    }
}
impl From<&str> for LogValue {
    fn from(s: &str) -> Self {
        LogValue::String(s.to_string())
    }
}
impl From<f64> for LogValue {
    fn from(n: f64) -> Self {
        LogValue::Number(n)
    }
}
impl From<bool> for LogValue {
    fn from(b: bool) -> Self {
        LogValue::Bool(b)
    }
}

impl ActiveRun {
    pub fn log(&self, _metrics: HashMap<impl Into<String>, impl Into<LogValue>>) {
        let _map = _metrics
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect::<HashMap<String, LogValue>>();
        unimplemented!()
    }

    pub async fn finish(self) -> FinishedRun {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct InitArgs {
    pub entity_name: Option<String>,
    pub project_name: Option<String>,
    // pub offline: bool,
}

impl Default for InitArgs {
    fn default() -> Self {
        Self {
            entity_name: None,
            project_name: None,
            // offline: false,
        }
    }
}

pub async fn init(args: InitArgs) -> api::ApiResult<ActiveRun> {
    let api = api::Api::default();
    let resp = api.upsert_bucket(args.entity_name, args.project_name).await?
        .upsert_bucket.ok_or("no bucket")?
        .bucket.ok_or("no bucket")?;
    let project = resp.project.ok_or("no project")?;
    Ok(ActiveRun {
        entity_name: project.entity.name,
        project_name: project.name,
        run_name: resp.name,
        _phantom: std::marker::PhantomData,
    })
}
