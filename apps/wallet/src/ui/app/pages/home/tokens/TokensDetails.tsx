// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { useMemo } from 'react';

import { TokenIconLink } from './TokenIconLink';
import CoinBalance from './coin-balance';
import IconLink from './icon-link';
import FaucetRequestButton from '_app/shared/faucet/request-button';
import PageTitle from '_app/shared/page-title';
import Alert from '_components/alert';
import Loading from '_components/loading';
import RecentTransactions from '_components/transactions-card/RecentTransactions';
import { HaneulIcons } from '_font-icons/output/haneul-icons';
import { useAppSelector, useObjectsState } from '_hooks';
import { accountAggregateBalancesSelector } from '_redux/slices/account';
import { GAS_TYPE_ARG, Coin } from '_redux/slices/haneul-objects/Coin';
import { AccountSelector } from '_src/ui/app/components/AccountSelector';

import st from './TokensPage.module.scss';

type TokenDetailsProps = {
    coinType?: string;
};

const emptyWalletDescription = (
    <div className={st.emptyWalletDescription}>
        To conduct transactions on the Haneul network, you need HANEUL in your wallet.
    </div>
);

type TokensProps = {
    allCoinTypes: string[];
    coinBalance: bigint;
    balances: Record<string, bigint>;
    loading: boolean;
};

function MyTokens({
    allCoinTypes,
    coinBalance,
    balances,
    loading,
}: TokensProps) {
    return (
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
                        {coinBalance <= 0 ? (
                            <div className={st.emptyWallet}>
                                <FaucetRequestButton trackEventSource="home" />
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
                    <FaucetRequestButton trackEventSource="home" />
                </div>
            )}
        </Loading>
    );
}

function TokenDetails({ coinType }: TokenDetailsProps) {
    const { loading, error, showError } = useObjectsState();
    const activeCoinType = coinType || GAS_TYPE_ARG;
    const accountAddress = useAppSelector(({ account }) => account.address);
    const balances = useAppSelector(accountAggregateBalancesSelector);
    const tokenBalance = balances[activeCoinType] || BigInt(0);
    const allCoinTypes = useMemo(() => Object.keys(balances), [balances]);
    const coinTypeWithBalance =
        coinType || tokenBalance > 0 ? activeCoinType : allCoinTypes[0];

    const coinSymbol = useMemo(
        () => Coin.getCoinSymbol(activeCoinType),
        [activeCoinType]
    );

    return (
        <>
            {coinType && (
                <PageTitle
                    title={coinSymbol}
                    backLink="/tokens"
                    hideBackLabel={true}
                />
            )}

            <div className={st.container} data-testid="coin-page">
                {showError && error ? (
                    <Alert className={st.alert}>
                        <div>
                            <strong>Sync error (data might be outdated)</strong>
                        </div>
                        <small>{error.message}</small>
                    </Alert>
                ) : null}
                {!coinType && <AccountSelector />}
                <div className={st.balanceContainer}>
                    <Loading loading={loading}>
                        <CoinBalance
                            balance={tokenBalance}
                            type={activeCoinType}
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

                {activeCoinType === GAS_TYPE_ARG && accountAddress ? (
                    <TokenIconLink accountAddress={accountAddress} />
                ) : null}

                {!coinType ? (
                    <MyTokens
                        allCoinTypes={allCoinTypes}
                        coinBalance={tokenBalance}
                        balances={balances}
                        loading={loading}
                    />
                ) : (
                    <>
                        <div className={cl([st.title, st.tokenActivities])}>
                            {coinSymbol} activity
                        </div>
                        <div className={st.txContent}>
                            <RecentTransactions coinType={activeCoinType} />
                        </div>
                    </>
                )}
            </div>
        </>
    );
}

export default TokenDetails;
