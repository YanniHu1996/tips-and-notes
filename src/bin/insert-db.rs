use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, ListParams, AttachParams};

use kube::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the environment to find config for kube client.
    // Note that this tries an in-cluster configuration first,
    // then falls back on a kubeconfig file.
    let client = Client::try_default().await?;

    // Interact with pods in the configured namespace with the typed interface from k8s-openapi
    let pods: Api<Pod> = Api::default_namespaced(client);

    // List pods in the configured namespace
    let lp = ListParams::default().labels("cnpg.io/cluster=test,cnpg.io/instanceRole=primary");
    for p in pods.list(&lp).await? {

        pods.exec(
            "example",
            vec!["sh", "-c", "for i in $(seq 1 3); do date; done"],
            &AttachParams::default().stderr(false),
        )
        .await?;
    }

    Ok(())
}
