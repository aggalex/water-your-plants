set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=192.168.88.239
readonly TARGET_PATH=/home/alex/Downloads/client/client
readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly SOURCE_PATH=../target/${TARGET_ARCH}/release/client

cargo zigbuild --release --target=${TARGET_ARCH} --features bin
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
rsync ./.env ${TARGET_HOST}:"$(dirname ${TARGET_PATH})/.env"
ssh -t ${TARGET_HOST} ${TARGET_PATH}