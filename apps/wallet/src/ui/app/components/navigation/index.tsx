// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { memo } from 'react';
import { NavLink } from 'react-router-dom';

import Icon, { HaneulIcons } from '_components/icon';
import { useAppSelector } from '_hooks';
import { getNavIsVisible } from '_redux/slices/app';

import st from './Navigation.module.scss';

function makeLinkCls({ isActive }: { isActive: boolean }) {
    return cl(st.link, { [st.active]: isActive });
}

export type NavigationProps = {
    className?: string;
};

function Navigation({ className }: NavigationProps) {
    const isVisible = useAppSelector(getNavIsVisible);
    return (
        <nav
            className={cl(st.container, className, { [st.hidden]: !isVisible })}
        >
            <div id="haneul-apps-filters"></div>

            <div className={st.navMenu}>
                <NavLink to="./tokens" className={makeLinkCls} title="Tokens">
                    <Icon className={st.icon} icon={HaneulIcons.Tokens} />
                    <span className={st.title}>Coins</span>
                </NavLink>
                <NavLink to="./nfts" className={makeLinkCls} title="NFTs">
                    <Icon className={st.icon} icon={HaneulIcons.Nfts} />
                    <span className={st.title}>NFTs</span>
                </NavLink>
                <NavLink to="./apps" className={makeLinkCls} title="Apps">
                    <Icon
                        className={cl(st.icon, st.appsIcon)}
                        icon={HaneulIcons.Apps}
                    />
                    <span className={st.title}>Apps</span>
                </NavLink>
                <NavLink
                    to="./transactions"
                    className={makeLinkCls}
                    title="Transactions"
                >
                    <Icon
                        className={cl(st.icon, st.walletActivityIcon)}
                        icon={HaneulIcons.Activity}
                    />
                    <span className={st.title}>Activity</span>
                </NavLink>
            </div>
        </nav>
    );
}

export default memo(Navigation);
