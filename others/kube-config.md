
## Environment variables locating kube config `KUBECONFIG`


## Login all or a specified k8s cluster(s) through tsh 

```sh
KUBECONFIG=<path_to_config_file> tsh kube login --proxy=<host>:<port> --auth=<auther> --all
KUBECONFIG=<path_to_config_file> tsh kube login --proxy=<host>:<port> --auth=<author> <cluste_name>
```

## List all clusters 

```sh
# By tsh
tsh kube ls 
KUBECONFIG=~/.kube/config  k config  get-contexts  -oname

# By kubectl
kubectl config get-clusters
```


## Switch context and namespace


### By Config 
```sh
kubectl config use-context <context>
kubectl config set-context --current --namesoace <namespace>
```

### By Kubie
```sh
kubie ctx -f <path_config_file> <context>

kubie ns <namespace> 

kubie export <context> <namespace>
```



## Reference 

https://kubernetes.io/docs/tasks/access-application-cluster/configure-access-multiple-clusters/#define-clusters-users-and-contexts
