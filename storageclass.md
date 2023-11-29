


```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: standard
provisioner: kubernetes.io/aws-ebs
parameters:
  type: gp2
reclaimPolicy: Retain
allowVolumeExpansion: true
mountOptions:
  - debug
volumeBindingMode: Immediate
```

allowVolumeExpansion:

> PersistentVolumes can be configured to be expandable. This feature when set to true, allows the users to resize the volume by editing the corresponding PVC object.
> The following types of volumes support volume expansion, when the underlying StorageClass has the field allowVolumeExpansion set to true.

### reference 

https://kubernetes.io/docs/concepts/storage/storage-classes/#allow-volume-expansion
