set -ex

. $(dirname $0)/env.sh

install() {
    local slug=$1 tag=$2
    local app=$(echo $slug | cut -d'/' -f2)
    local url=https://github.com/$slug/releases/download/$tag/$app-$tag-$HOST.tar.gz \

    curl -sL $url | tar -C ~/.cargo/bin -xz
}

main() {
    if [[ $OSX || ${INSIDE_DOCKER_CONTAINER:-n} == y ]]; then
        if [[ $LINUX ]]; then
            rustup default nightly

            mkdir ghp-import
            curl -Ls https://github.com/davisp/ghp-import/archive/master.tar.gz | \
                tar --strip-components 1 -C ghp-import -xz
        fi

        if [[ $OSX ]]; then
            curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

            install japaric/xargo v0.1.7

            brew tap caskroom/cask
            brew cask install gcc-arm-embedded
        fi

        rustup component add rust-src

        install azerupi/mdBook v0.0.14
    fi
}

main
