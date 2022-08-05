// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject } from '@haneullabs/haneul.js';
import cl from 'classnames';
import { useMemo, useState, useCallback } from 'react';
import { Navigate, useSearchParams } from 'react-router-dom';

import TransferNFTCard from './transfer-nft';
import BottomMenuLayout, {
    Content,
    Menu,
} from '_app/shared/bottom-menu-layout';
import PageTitle from '_app/shared/page-title';
import ExplorerLink from '_components/explorer-link';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';
import Icon, { HaneulIcons } from '_components/icon';
import Loading from '_components/loading';
import NFTDisplayCard from '_components/nft-display';
import {
    useAppSelector,
    useFileExtentionType,
    useMiddleEllipsis,
} from '_hooks';
import { accountNftsSelector } from '_redux/slices/account';

import type { HaneulObject } from '@haneullabs/haneul.js';

import st from './NFTDetails.module.scss';

function NFTDetailsPage() {
    const [searchParams] = useSearchParams();
    const [startNFTTransfer, setStartNFTTransfer] = useState<boolean>(false);
    const [selectedNFT, setSelectedNFT] = useState<HaneulObject | null>(null);
    const objectId = useMemo(
        () => searchParams.get('objectId'),
        [searchParams]
    );

    let nftFields;
    const nftCollections = useAppSelector(accountNftsSelector);

    const activeNFT = useMemo(() => {
        const r = nftCollections.filter(
            (nftItems) => nftItems.reference.objectId === objectId
        )[0];
        setSelectedNFT(r);
        return r;
    }, [nftCollections, objectId]);

    if (activeNFT) {
        nftFields = isHaneulMoveObject(activeNFT.data)
            ? activeNFT.data.fields
            : null;
    }

    const loadingBalance = useAppSelector(
        ({ haneulObjects }) => haneulObjects.loading && !haneulObjects.lastSync
    );

    const startNFTTransferHandler = useCallback(() => {
        setStartNFTTransfer(true);
    }, []);

    const shortAddress = useMiddleEllipsis(nftFields?.info.id, 10, 6);
    const fileExtentionType = useFileExtentionType(nftFields?.url || '');

    if (!objectId || (!loadingBalance && !selectedNFT && !startNFTTransfer)) {
        return <Navigate to="/nfts" replace={true} />;
    }

    const NFTDetails = nftFields && (
        <div className={st.nftDetails}>
            <div className={st.nftItemDetail}>
                <div className={st.label}>Object ID</div>
                <div className={st.value}>
                    <ExplorerLink
                        type={ExplorerLinkType.address}
                        address={nftFields.info.id}
                        title="View on Haneul Explorer"
                        className={st.explorerLink}
                        showIcon={false}
                    >
                        {shortAddress}
                    </ExplorerLink>
                </div>
            </div>

            <div className={st.nftItemDetail}>
                <div className={st.label}>Media Type</div>
                <div className={st.value}>{fileExtentionType}</div>
            </div>
        </div>
    );

    const NFTdetailsContent = (
        <div className={st.container}>
            <PageTitle
                title={nftFields?.name}
                backLink="/nfts"
                className={st.pageTitle}
                hideBackLabel={true}
            />
            <BottomMenuLayout>
                <Content>
                    <section className={st.nftDetail}>
                        {selectedNFT && (
                            <NFTDisplayCard
                                nftobj={selectedNFT}
                                size="large"
                                expandable={true}
                            />
                        )}
                        {NFTDetails}
                    </section>
                </Content>
                <Menu stuckClass={st.shadow} className={st.shadow}>
                    <button
                        onClick={startNFTTransferHandler}
                        className={cl(
                            'btn',
                            st.action,
                            st.sendNftBtn,
                            'primary'
                        )}
                    >
                        <Icon
                            icon={HaneulIcons.ArrowLeft}
                            className={cl(st.arrowActionIcon, st.angledArrow)}
                        />
                        Send NFT
                    </button>
                </Menu>
            </BottomMenuLayout>
        </div>
    );

    return (
        <Loading loading={loadingBalance}>
            {objectId && startNFTTransfer ? (
                <TransferNFTCard objectId={objectId} />
            ) : (
                NFTdetailsContent
            )}
        </Loading>
    );
}

export default NFTDetailsPage;
