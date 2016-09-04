main() {
    if [[ ${INSIDE_DOCKER_CONTAINER:-n} == y ]]; then
        rustup default nightly
    fi
}

main
