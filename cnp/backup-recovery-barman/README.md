
```sh
export AWS_ACCESS_KEY_ID=minio
export AWS_SECRET_ACCESS_KEY=minio123
barman-cloud-backup-list  --endpoint-url http://minio-service.minio:9000  --cloud-provider aws-s3 s3://backup/ test

```

