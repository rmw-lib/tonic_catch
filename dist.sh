#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

git pull

./clippy.sh

cargo set-version --bump patch

version=v$(cat Cargo.toml|grep "^version"|awk -F\" '{print $2}')

git add -u
git commit -m $version
git tag $version
git push origin $version
git push

cargo +nightly publish

