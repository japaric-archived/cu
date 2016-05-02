#!/bin/bash

set -ex

main() {
    local tag=2016-04-26

    # The particle user has id = 1000, but this may not match the travis user id. To workaround this
    # issue, make everything world write-able.
    chmod -R a+w .

    docker run \
           -v $(pwd):/mnt \
           -w /mnt \
           -e AR_thumbv7m_none_eabi=arm-none-eabi-ar \
           -e CC_thumbv7m_none_eabi=arm-none-eabi-gcc \
           japaric/copper:$tag \
           bash -ex -c '
rustup default nightly
xargo build --release --verbose
arm-none-eabi-size $(find target/thumbv7m-none-eabi/release -maxdepth 1 -type f -executable)
arm-none-eabi-objdump -CD $(find target/thumbv7m-none-eabi/release -maxdepth 1 -type f -executable)
    '
}

main
