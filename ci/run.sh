set -ex

test_mode() {
    for app in $(echo app/*); do
        local target=
        case $app in
            app/01-qemu)
                target=thumbv7m-none-eabi
                ;;
            *)
                target=thumbv7em-none-eabihf
                ;;
        esac

        cd $app
        xargo build --target $target
        arm-none-eabi-objdump -Cd target/$target/debug/app
        cd ../..
    done

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        mdbook build
    fi
}

run() {
    test_mode
}

run
