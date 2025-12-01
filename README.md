# EC2-Proto

cloud-hypervisor(https://github.com/cloud-hypervisor/cloud-hypervisor)

## setup

CLOUDHV.fdを[CHのedk2]("https://github.com/cloud-hypervisor/edk2/releases/tag/ch-a54f262b09")からダウンロードしてimages/に置く
クラウド用のnobleを同じくimage/に置く

## boot

`./cloud-hypervisor --api-socket /tmp/cloud-hypervisor.sock`

``` sh
curl --unix-socket /tmp/cloud-hypervisor.sock -i \
     -X PUT 'http://localhost/api/v1/vm.create'  \
     -H 'Accept: application/json'               \
     -H 'Content-Type: application/json'         \
     -d '{
         "cpus":{"boot_vcpus": 1, "max_vcpus": 1},
         "payload": {"kernel":"./images/CLOUDHV.fd"},
         "cmdline":{"args":"console=ttyS0 console=hvc0 root=/dev/vda1 rw"},
         "disks":[{"path":"./images/noble.raw"}],
         "rng":{"src":"/dev/urandom"}
         }'
```

`curl --unix-socket /tmp/cloud-hypervisor.sock -i -X PUT 'http://localhost/api/v1/vm.boot'`
