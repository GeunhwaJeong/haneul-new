// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { memo } from 'react';

import Icon, { HaneulIcons } from '_components/icon';
import { useFormatCoin } from '_hooks';
import { GAS_TYPE_ARG } from '_redux/slices/haneul-objects/Coin';

import st from './CoinBalance.module.scss';

export type CoinProps = {
    type: string;
    balance: bigint;
    hideStake?: boolean;
    mode?: 'row-item' | 'standalone';
};

function CoinBalance({ type, balance, mode = 'row-item' }: CoinProps) {
    const [formatted, symbol] = useFormatCoin(balance, type);
    const icon = type === GAS_TYPE_ARG ? HaneulIcons.HaneulLogoIcon : HaneulIcons.Tokens;
    return (
        <div className={cl(st.container, st[mode])}>
            {mode === 'row-item' ? (
                <>
                    <Icon
                        icon={icon}
                        className={cl(st.coinIcon, {
                            [st.haneul]: type === GAS_TYPE_ARG,
                        })}
                    />
                    <div className={cl(st.coinNameContainer, st[mode])}>
                        <span className={st.coinName}>
                            {symbol.toLocaleLowerCase()}
                        </span>
                        <span className={st.coinSymbol}>{symbol}</span>
                    </div>
                </>
            ) : null}
            <div className={cl(st.valueContainer, st[mode])}>
                <span className={cl(st.value, st[mode])}>{formatted}</span>
                <span className={cl(st.symbol, st[mode])}>{symbol}</span>
            </div>
        </div>
    );
}

export default memo(CoinBalance);
