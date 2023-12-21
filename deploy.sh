set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=192.168.88.239
readonly TARGET_PATH=/home/alex/Downloads/deployPi
readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/deployPi

cargo zigbuild --release --target=${TARGET_ARCH}
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}