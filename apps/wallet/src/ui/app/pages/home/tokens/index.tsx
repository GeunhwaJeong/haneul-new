// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

import CoinBalance from './coin-balance';
import IconLink from './icon-link';
import AccountAddress from '_components/account-address';
import Alert from '_components/alert';
import Loading from '_components/loading';
import { HaneulIcons } from '_font-icons/output/haneul-icons';
import { useAppSelector, useObjectsState } from '_hooks';
import { accountAggregateBalancesSelector } from '_redux/slices/account';
import { GAS_TYPE_ARG } from '_redux/slices/haneul-objects/Coin';

import st from './TokensPage.module.scss';

function TokensPage() {
    const { loading, error, showError } = useObjectsState();
    const balances = useAppSelector(accountAggregateBalancesSelector);
    const haneulBalance = balances[GAS_TYPE_ARG] || BigInt(0);
    const otherCoinTypes = useMemo(
        () => Object.keys(balances).filter((aType) => aType !== GAS_TYPE_ARG),
        [balances]
    );
    return (
        <div className={st.container}>
            {showError && error ? (
                <Alert className={st.alert}>
                    <strong>Sync error (data might be outdated).</strong>{' '}
                    <small>{error.message}</small>
                </Alert>
            ) : null}
            <AccountAddress showLink={false} mode="faded" />
            <div className={st.balanceContainer}>
                <Loading loading={loading}>
                    <CoinBalance
                        balance={haneulBalance}
                        type={GAS_TYPE_ARG}
                        mode="standalone"
                    />
                </Loading>
            </div>
            <div className={st.actions}>
                <IconLink
                    icon={HaneulIcons.Buy}
                    to="/"
                    disabled={true}
                    text="Buy"
                />
                <IconLink
                    icon={HaneulIcons.ArrowLeft}
                    to={`/send?${new URLSearchParams({
                        type: GAS_TYPE_ARG,
                    }).toString()}`}
                    text="Send"
                />
                <IconLink
                    icon={HaneulIcons.Swap}
                    to="/"
                    disabled={true}
                    text="Swap"
                />
            </div>
            <div className={st.staking}>
                <IconLink
                    icon={HaneulIcons.Union}
                    to="/stake"
                    disabled={true}
                    text="Stake & Earn HANEUL"
                />
            </div>
            <div className={st.title}>OTHER COINS</div>
            <div className={st.otherCoins}>
                <Loading loading={loading} className={st.othersLoader}>
                    {otherCoinTypes.length ? (
                        otherCoinTypes.map((aCoinType) => {
                            const aCoinBalance = balances[aCoinType];
                            return (
                                <CoinBalance
                                    type={aCoinType}
                                    balance={aCoinBalance}
                                    key={aCoinType}
                                />
                            );
                        })
                    ) : (
                        <div className={st.empty}>
                            No coins have added. When you have multiple coins in
                            your wallet, they will be listed here.
                        </div>
                    )}
                </Loading>
            </div>
        </div>
    );
}

export default TokensPage;
