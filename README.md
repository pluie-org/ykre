# Ykre

**Ykre** is a small cli program written in rust wich purpose is to extract from a list
of yaml documents, documents matching a specified condition. **Ykre** goal is to find 
specific Kubernetes Resource from the kustomize output.
**Ykre** stands for **Y**aml **K**ubernetes **R**esources **E**xtractor.


## Install

from a debian package :
``` 
wget -q https://github.com/pluie-org/ykre/raw/master/dist/ykre_0.2.0_amd64.deb
sudo apt install ./ykre_0.2.0_amd64.deb
```

via Cargo
``` 
wget -q -O - https://github.com/pluie-org/ykre/raw/master/install.sh | sh
```


## Usage
```
ykre SEARCH [DEF]
```

## examples

display content of the kubernetes resource 'myResourceName'
```
cat kubresources.yaml | ykre 'myResourceName'
```
write content of the kubernetes resource 'pv-dump' into a file
```
kustomize build ./config/volumes/local | ykre 'pv-dump' > ./tmp.yaml
```
retriew kubernetes pv resources matching 2Gi disk capacity
```
kustomize build ./config/volumes/dev | ykre 2Gi spec.capacity.storage
```


## License

MIT
