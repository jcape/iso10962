#!/bin/bash

mkdir -p /workspaces/iso10962/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso10962/.cache/cargo/

cargo binstall -q -y --force prek
cargo binstall -q -y --force action-validator
cargo binstall -q -y --force cargo-deny
cargo binstall -q -y --force cargo-nextest

pushd /workspaces/iso10962 >/dev/null
prek install >/dev/null
popd >/dev/null
