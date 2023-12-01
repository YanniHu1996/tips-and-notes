


```sh
k patch cluster $name --type='json' -p='[{"op": "replace", "path": "/spec/instances", "value": $num}]'
```
