#!/bin/sh

set -ex

main() {
    local svd=nrf51.svd

    if [ ! -f $svd ]; then
        curl -LOs https://github.com/posborne/cmsis-svd/raw/python-0.4/data/Nordic/$svd
    fi

    svd2rust -i $svd gpio > src/gpio.rs
    svd2rust -i $svd timer > src/timer.rs
    sed -i 's/\(pub struct Timer\)0/\1/' src/timer.rs

    set +e
    rustfmt src/*.rs
    set -e

    xargo build --target thumbv6m-none-eabi
}

main
