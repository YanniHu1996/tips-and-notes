use futures_util::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, AttachParams, AttachedProcess, ListParams};
use kube::client::Client;
use kube::ResourceExt;

const LABEL_CLUSTER: &str = "cnpg.io/cluster";
const LABEL_INSTANCE_ROLE: &str = "cnpg.io/instanceRole";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cluster_name = "test";

    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    let lp = ListParams::default().labels(&format!(
        "{}={},{}=primary",
        LABEL_CLUSTER, cluster_name, LABEL_INSTANCE_ROLE
    ));
    let pod_list = pods.list(&lp).await?;
    let primary_instance = pod_list.items.first().unwrap();

    let attached: AttachedProcess = pods
        .exec(
            primary_instance.name_any().as_str(),
            vec!["psql", "-c", "select 1;"],
            &AttachParams::default().stderr(false),
        )
        .await?;

    let output = get_output(attached).await;
    println!("{output}");

    Ok(())
}

async fn get_output(mut attached: AttachedProcess) -> String {
    let stdout = tokio_util::io::ReaderStream::new(attached.stdout().unwrap());
    let out = stdout
        .filter_map(|r| async { r.ok().and_then(|v| String::from_utf8(v.to_vec()).ok()) })
        .collect::<Vec<_>>()
        .await
        .join("");
    attached.join().await.unwrap();
    out
}
