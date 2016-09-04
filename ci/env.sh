case $TRAVIS_OS_NAME in
    linux)
        HOST=x86_64-unknown-linux-gnu
        LINUX=1
        ;;
    osx)
        HOST=x86_64-apple-darwin
        OSX=1
        ;;
esac

export PATH="$HOME/.cargo/bin:$PATH"
