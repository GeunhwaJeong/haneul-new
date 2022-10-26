// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

import CoinBalance from './coin-balance';
import IconLink from './icon-link';
import FaucetMessageInfo from '_app/shared/faucet/message-info';
import FaucetRequestButton from '_app/shared/faucet/request-button';
import AccountAddress from '_components/account-address';
import Alert from '_components/alert';
import Loading from '_components/loading';
import { HaneulIcons } from '_font-icons/output/haneul-icons';
import { useAppSelector, useObjectsState } from '_hooks';
import { accountAggregateBalancesSelector } from '_redux/slices/account';
import { GAS_TYPE_ARG } from '_redux/slices/haneul-objects/Coin';

import st from './TokensPage.module.scss';

const emptyWalletDescription = (
    <div className={st.emptyWalletDescription}>
        To conduct transactions on the Haneul network, you need HANEUL in your wallet.
    </div>
);

function TokensPage() {
    const { loading, error, showError } = useObjectsState();
    const balances = useAppSelector(accountAggregateBalancesSelector);
    const haneulBalance = balances[GAS_TYPE_ARG] || BigInt(0);
    const allCoinTypes = useMemo(() => Object.keys(balances), [balances]);
    const coinTypeWithBalance = haneulBalance > 0 ? GAS_TYPE_ARG : allCoinTypes[0];
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
                    to={`/send${
                        coinTypeWithBalance
                            ? `?${new URLSearchParams({
                                  type: coinTypeWithBalance,
                              }).toString()}`
                            : ''
                    }`}
                    disabled={!coinTypeWithBalance}
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
            <Loading loading={loading} className={st.othersLoader}>
                {allCoinTypes.length ? (
                    <>
                        <div className={st.title}>MY COINS</div>
                        <div className={st.otherCoins}>
                            {allCoinTypes.map((aCoinType) => (
                                <CoinBalance
                                    type={aCoinType}
                                    balance={balances[aCoinType] || BigInt(0)}
                                    key={aCoinType}
                                />
                            ))}
                            {haneulBalance <= 0 ? (
                                <div className={st.emptyWallet}>
                                    <FaucetRequestButton />
                                    {emptyWalletDescription}
                                </div>
                            ) : null}
                        </div>
                    </>
                ) : (
                    <div className={st.emptyWallet}>
                        <div className={st.emptyWalletIcon} />
                        <div className={st.emptyWalletTitle}>
                            Your wallet contains no HANEUL.
                        </div>
                        {emptyWalletDescription}
                        <FaucetRequestButton />
                    </div>
                )}
                <FaucetMessageInfo className={st.gasRequestInfo} />
            </Loading>
        </div>
    );
}

export default TokensPage;
