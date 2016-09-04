#!/bin/bash

set -ex

. $(dirname $0)/env.sh

build() {
    xargo build --release --verbose
}

inspect() {
    local elfs=$(find target/cortex-m3/release -maxdepth 1 -type f -executable)

    for elf in $elfs; do
        arm-none-eabi-readelf -h $elf | grep "Entry point address" | grep -v 0x0$;
    done
    arm-none-eabi-size $elfs
    arm-none-eabi-objdump -CD $elfs
}

main() {
    if [[ ${INSIDE_DOCKER_CONTAINER:-n} == n ]]; then
        local gid=$(id -g) \
              id=rust \
              uid=$(id -u)

        docker run \
               --entrypoint bash \
               -e INSIDE_DOCKER_CONTAINER=y \
               -v $(pwd):/mnt \
               japaric/copper \
               -c "
set -eux
usermod -u $uid $id
groupmod -g $gid $id
chgrp -R $id /home/$id
HOME=/home/$id USER=$id su -c 'cd /mnt && bash ci/install.sh && bash ci/script.sh' $id
"
    else
        build
        inspect
    fi
}

main
