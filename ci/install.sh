set -ex

. $(dirname $0)/env.sh

main() {
    if [[ ${INSIDE_DOCKER_CONTAINER:-n} == y ]]; then
        rustup default nightly
    fi
}

main
