set -ex

build_apps() {
    local tag=2016-05-10
    local target=cortex-m3

    # The copper user has id = 1000, but this may not match the travis user id. To workaround this
    # issue, make everything world write-able.
    chmod -R a+w .

    docker run \
           -v $(pwd):/mnt \
           -w /mnt \
           japaric/copper:$tag \
           bash -ex -c "
for app in $(echo app/*); do
    pushd \$app
    xargo build --target $target
    arm-none-eabi-objdump -Cd -j .vector_table -j .text target/$target/debug/app
    popd
done
"
}

build_docs() {
    mdbook build
}

main() {
    build_apps
    build_docs
}

main
