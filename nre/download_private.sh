#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

if ! cosign version &> /dev/null
then
    echo "cosign in not installed, Please install cosign for binary verification."
    echo "https://docs.sigstore.dev/cosign/installation"
    exit
fi

commit_sha=$1
pub_key=https://haneul-private.s3.us-west-2.amazonaws.com/haneul_security_release.pem
url=https://haneul-releases.s3-accelerate.amazonaws.com/$commit_sha

echo "[+] Downloading haneul binaries for $commit_sha ..."
curl $url/haneul -o haneul
curl $url/haneul-indexer -o haneul-indexer
curl $url/haneul-node -o haneul-node
curl $url/haneul-tool -o haneul-tool

echo "[+] Verifying haneul docker artifacts for $commit_sha ..."
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul.sig haneul
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul-indexer.sig haneul-indexer
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/haneul-node.sig haneul-node

