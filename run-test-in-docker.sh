#!/bin/bash

if [ ! -n "$1" ]; then
    echo "usage: docker run ... [package]{1} [params]{0..n}"
    exit 1
fi

package=$1
shift
params=$@

cargo test --package $package $params