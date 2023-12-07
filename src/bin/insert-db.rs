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
    let pod_api: Api<Pod> = Api::default_namespaced(client);

    let primary_instance = get_pod(&pod_api, cluster_name).await;

    let output = exec_sql(&pod_api, &primary_instance, "SELECT 1;").await;
    println!("Output: {}", output);

    Ok(())
}

async fn get_pod(pod_api: &Api<Pod>, cluster_name: &str) -> Pod {
    let lp = ListParams::default().labels(&format!(
        "{}={},{}=primary",
        LABEL_CLUSTER, cluster_name, LABEL_INSTANCE_ROLE
    ));
    let pod_list = pod_api.list(&lp).await.unwrap();
    pod_list.items.first().unwrap().to_owned()
}

async fn exec_sql(pod_api: &Api<Pod>, pod: &Pod, sql: &str) -> String {
    let attached = pod_api
        .exec(
            pod.name_any().as_str(),
            vec!["psql", "-c", sql],
            &AttachParams::default().stderr(false),
        )
        .await
        .unwrap();

    get_output(attached).await
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
