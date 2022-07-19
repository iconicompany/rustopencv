#build tesseract 5.1.0 (change version to force rebuild)
set -e
# see add variants: https://notesalexp.org/
DEBVER="${1:-bullseye}"
echo "deb https://notesalexp.org/tesseract-ocr5/${DEBVER}/ ${DEBVER} main" >> /etc/apt/sources.list
apt-get update -oAcquire::AllowInsecureRepositories=true
apt-get install -y -qq --no-install-recommends --purge --allow-unauthenticated notesalexp-keyring -oAcquire::AllowInsecureRepositories=true
apt-get update
apt-get install -y -qq --no-install-recommends --purge --allow-unauthenticated -t ${DEBVER} tesseract-ocr libtesseract-dev
