use futures::StreamExt;
use kube::{
    Api, Client,
    runtime::controller::{Action, Controller},
    api::{Patch, PatchParams},
    Config,
};
use std::sync::Arc;
use tokio::time::Duration;

mod error;
use error::Error;

// Import from lib.rs
use tokio_hello::{Pinger, PingerStatus};

// Context data that will be shared with the reconciliation function
#[derive(Clone)]
struct ContextData {
    client: Client,
}

// The reconciliation function
async fn reconcile(pinger: Arc<Pinger>, ctx: Arc<ContextData>) -> Result<Action, Error> {
    let client = &ctx.client;
    let name = pinger.metadata.name.as_deref().unwrap_or("unknown");
    let namespace = pinger.metadata.namespace.as_deref().unwrap_or("default");
    let url = &pinger.spec.url;

    println!("Reconciling Pinger \"{}\" in {}", name, namespace);

    // Send a request to the specified URL
    let response = reqwest::get(url).await?;
    let status = response.status();
    println!("Status: {}", status);

    // Update the Pinger status
    // let pingers: Api<Pinger> = Api::namespaced(client.clone(), namespace);
    // let patch: Patch<serde_json::Value> = Patch::Merge(serde_json::json!({
    //     "status": PingerStatus {
    //         last_ping: Some(format!("Pinged {} with status {}", url, status)),
    //     }
    // }));
    // pingers.patch_status(name, &PatchParams::default(), &patch).await?;

    // Reconcile again after 2 seconds
    Ok(Action::requeue(Duration::from_secs(2)))
}

// The error policy function
fn error_policy(_pinger: Arc<Pinger>, error: &Error, _ctx: Arc<ContextData>) -> Action {
    eprintln!("Reconciliation error: {:?}", error);
    Action::requeue(Duration::from_secs(5))
}

// The main function
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get the local Kubernetes configuration
    let config = Config::infer().await?;
    let client = Client::try_from(config)?;
    
    // Create an API for Pinger resources
    let pingers: Api<Pinger> = Api::all(client.clone());
    
    // Create the context data
    let context = Arc::new(ContextData { client: client.clone() });

    println!("Starting to watch Pinger resources in all namespaces...");

    // Create and run the controller
    Controller::new(pingers, Default::default())
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => println!("Reconciliation successful: {:?}", o),
                Err(e) => eprintln!("Reconciliation error: {:?}", e),
            }
        })
        .await;

    Ok(())
}
