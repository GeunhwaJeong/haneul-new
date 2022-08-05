// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { isHaneulMoveObject } from '@haneullabs/haneul.js';
import cl from 'classnames';

import ExplorerLink from '_components/explorer-link';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';
import Icon, { HaneulIcons } from '_components/icon';
import { useFileExtentionType, useMediaUrl } from '_hooks';

import type { HaneulObject as HaneulObjectType } from '@haneullabs/haneul.js';

import st from './NFTDisplay.module.scss';

export type NFTsProps = {
    nftobj: HaneulObjectType;
    showlabel?: boolean;
    size?: 'small' | 'medium' | 'large';
    expandable?: boolean;
    wideview?: boolean;
};

function NFTDisplayCard({
    nftobj,
    showlabel,
    size = 'medium',
    expandable,
    wideview,
}: NFTsProps) {
    const imgUrl = useMediaUrl(nftobj.data);
    const nftFields = isHaneulMoveObject(nftobj.data) ? nftobj.data.fields : null;
    const fileExtentionType = useFileExtentionType(nftFields?.url || '');

    const wideviewSection = (
        <div className={st.nftfields}>
            <div className={st.nftName}>{nftFields?.name}</div>
            <div className={st.nftType}>{fileExtentionType}</div>
        </div>
    );
    const defaultSection = (
        <>
            {expandable && nftFields?.info.id ? (
                <div className={st.expandable}>
                    <ExplorerLink
                        type={ExplorerLinkType.object}
                        objectID={nftFields?.info.id}
                        showIcon={false}
                        className={st['explorer-link']}
                    >
                        View Image <Icon icon={HaneulIcons.Preview} />
                    </ExplorerLink>
                </div>
            ) : null}
            {showlabel && nftFields?.name ? (
                <div className={st.nftfields}>{nftFields.name}</div>
            ) : null}
        </>
    );

    return (
        <div className={cl(st.nftimage, wideview && st.wideview)}>
            {imgUrl && (
                <img className={cl(st.img, st[size])} src={imgUrl} alt="NFT" />
            )}
            {wideview ? wideviewSection : defaultSection}
        </div>
    );
}

export default NFTDisplayCard;
