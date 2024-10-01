use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Kubernetes error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Config inference error: {0}")]
    InferConfigError(#[from] kube::config::InferConfigError),
}