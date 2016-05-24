#!/bin/bash

set -ex

main() {
    local tag=2016-05-10

    # The particle user has id = 1000, but this may not match the travis user id. To workaround this
    # issue, make everything world write-able.
    chmod -R a+w .

    docker run \
           -v $(pwd):/mnt \
           -w /mnt \
           japaric/copper:$tag \
           bash -ex -c '
rustup default nightly
xargo build --release --verbose
for elf in $(find target/thumbv7m-none-eabi/release -maxdepth 1 -type f -executable); do
    arm-none-eabi-readelf -h $elf | grep "Entry point address" | grep -v 0x0$;
done
arm-none-eabi-size $(find target/thumbv7m-none-eabi/release -maxdepth 1 -type f -executable)
arm-none-eabi-objdump -CD $(find target/thumbv7m-none-eabi/release -maxdepth 1 -type f -executable)
'
}

main
