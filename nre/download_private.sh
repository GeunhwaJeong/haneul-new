#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

if ! cosign version &> /dev/null
then
    echo "cosign in not installed, Please install cosign for binary verification."
    echo "https://docs.sigstore.dev/cosign/installation"
    exit
fi

randomstring=$1
pub_key=https://haneul-private.s3.us-west-2.amazonaws.com/haneul_security_release.pem
url=https://haneul-private.s3.us-west-2.amazonaws.com/$randomstring

echo "[+] Downloading docker artifacts for $randomstring ..."
curl $url/haneul-node-docker.tar -o haneul-node-docker.tar
curl $url/haneul-tools-docker.tar -o haneul-tools-docker.tar
curl $url/haneul-indexer-docker.tar -o haneul-indexer-docker.tar

echo "[+] Verifying docker artifacts for $randomstring ..."
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul-node-docker.tar.sig haneul-node-docker.tar
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul-tools-docker.tar.sig haneul-tools-docker.tar
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul-indexer-docker.tar.sig haneul-indexer-docker.tar

echo "[+] Downloading haneul binaries for $randomstring ..."
curl $url/haneul -o haneul
curl $url/haneul-indexer -o haneul-indexer
curl $url/haneul-node -o haneul-node
curl $url/haneul-tool -o haneul-tool

echo "[+] Verifying haneul binaries for $randomstring ..."
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul.sig haneul
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul-indexer.sig haneul-indexer
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul-node.sig haneul-node

