
## source cluster

```yaml
apiVersion: postgresql.cnpg.io/v1
kind: Cluster
metadata:
  name: test
  namespace: default
spec:
  storage:
    size: 1Gi
    storageClass: csi-hostpath-sc
  backup:
    volumeSnapshot:
      className: csi-hostpath-snapclass
      online: true
      onlineConfiguration:
        waitForArchive: true
        immediateCheckpoint: true
```

## target cluster

```yaml
apiVersion: postgresql.cnpg.io/v1
kind: Cluster
metadata:
  name: test-from-vs
spec:
  bootstrap:
    recovery:
      volumeSnapshots:
        storage:
          name: test-backup
          kind: VolumeSnapshot
          apiGroup: snapshot.storage.k8s.io
```


todo... 
