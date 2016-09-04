set -ex

. $(dirname $0)/env.sh

build_apps() {
    local target=cortex-m3

    for app in $(echo app/*); do
        pushd $app
        xargo build --target $target
        arm-none-eabi-objdump -Cd -j .vector_table -j .text target/$target/debug/app
        popd
    done
}

build_docs() {
    mdbook build
}

main() {
    if [[ $LINUX && ${INSIDE_DOCKER_CONTAINER:-n} == n ]]; then
        local gid=$(id -g) \
              id=rust \
              uid=$(id -u)

        docker run \
               --entrypoint bash \
               -e INSIDE_DOCKER_CONTAINER=y \
               -e TRAVIS_OS_NAME=$TRAVIS_OS_NAME \
               -v $(pwd):/mnt \
               japaric/copper \
               -c "
set -eux
usermod -u $uid $id
groupmod -g $gid $id
chgrp -R $id /home/$id
HOME=/home/$id USER=$id su -c '
    cd /mnt && bash ci/install.sh && bash ci/script.sh && bash ci/after_success.sh
' $id
"
    else
        build_apps
        build_docs
    fi
}

main
