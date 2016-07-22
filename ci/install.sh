set -ex

case $TRAVIS_OS_NAME in
    linux)
        git clone --depth 1 https://github.com/davisp/ghp-import
        ;;
    osx)
        command -v xargo >/dev/null 2>&1 || cargo install xargo --debug
        brew install openocd qemu
    ;;
esac

# TODO use versioned release instead of git snapshot
command -v mdbook >/dev/null 2>&1 || cargo install --git https://github.com/azerupi/mdbook --debug --no-default-features --features output
