// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import ObjectsLayout from '_components/objects-layout';
import HaneulObject from '_components/haneul-object';
import { useAppSelector } from '_hooks';
import { accountNftsSelector } from '_redux/slices/account';

function NftsPage() {
    const nfts = useAppSelector(accountNftsSelector);
    return (
        <ObjectsLayout totalItems={nfts.length} emptyMsg="No NFTs found">
            {nfts.map((anNft) => (
                <HaneulObject
                    obj={anNft}
                    sendNFT={true}
                    key={anNft.reference.objectId}
                />
            ))}
        </ObjectsLayout>
    );
}

export default NftsPage;
