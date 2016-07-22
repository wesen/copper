set -ex

case $TRAVIS_OS_NAME in
    osx)
        command -v xargo >/dev/null 2>&1 || cargo install xargo --debug
        brew install openocd qemu
    ;;
esac

command -v ghp >/dev/null 2>&1 || cargo install ghp --debug

# TODO use versioned release instead of git snapshot
command -v mdbook >/dev/null 2>&1 || cargo install --git https://github.com/azerupi/mdbook --debug --no-default-features --features output
