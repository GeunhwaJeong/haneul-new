// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulObjectResponse, getObjectOwner } from '@haneullabs/haneul.js';
import { HaneulClient } from '@haneullabs/haneul.js/client';

// get NFT's owner from RPC.
export const getOwner = async (client: HaneulClient, nftId: string): Promise<string | null> => {
    const ownerResponse = await client.getObject({
        id: nftId,
        options: { showOwner: true },
    });
    const owner = getObjectOwner(ownerResponse);
    return (
        (owner as { AddressOwner: string })?.AddressOwner ||
        (owner as { ObjectOwner: string })?.ObjectOwner ||
        null
    );
};

// get avatar NFT Object from RPC.
export const getAvatar = async (client: HaneulClient, avatar: string): Promise<HaneulObjectResponse> => {
    return await client.getObject({
        id: avatar,
        options: {
            showDisplay: true,
            showOwner: true,
        },
    });
};
