


```sh
k apply -f minio/deploy.yaml
k -n minio wait deployment/minio --for=condition=Available=true
```
