// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import StakedCard from './staked-card';
import BottomMenuLayout, {
    Content,
    Menu,
} from '_app/shared/bottom-menu-layout';
import Button from '_app/shared/button';
import CoinBalance from '_app/shared/coin-balance';
import PageTitle from '_app/shared/page-title';
import StatsCard, { StatsRow, StatsItem } from '_app/shared/stats-card';
import Icon, { HaneulIcons } from '_components/icon';
import { GAS_SYMBOL } from '_redux/slices/haneul-objects/Coin';

import st from './StakeHome.module.scss';

const DEMO_STAKED = Array.from({ length: 7 }).map((_, index) => (
    <StakedCard
        key={index}
        apy={Math.floor(Math.random() * 3500) / 100}
        balance={BigInt(Math.floor(100 * Math.random() + 1))}
        symbol="HANEUL"
        validator={`Validator ${index + 1}`}
        rewards={!Math.floor(Math.random() + 0.5)}
    />
));

function StakeHome() {
    return (
        <div className={st.container}>
            <PageTitle title="Stake & Earn" className={st.pageTitle} />
            <BottomMenuLayout>
                <Content>
                    <div className={st.pageDescription}>
                        Staking HANEUL provides HANEUL holders with rewards in
                        addition to market price gains.
                    </div>
                    <StatsCard className={st.stats}>
                        <StatsRow>
                            <StatsItem
                                title="Total Staked"
                                value={
                                    <CoinBalance
                                        balance={BigInt(50)}
                                        symbol={GAS_SYMBOL}
                                        diffSymbol={true}
                                    />
                                }
                            />
                            <StatsItem
                                title="Rewards Collected"
                                value={
                                    <CoinBalance
                                        balance={BigInt(12)}
                                        symbol={GAS_SYMBOL}
                                        mode="positive"
                                        diffSymbol={true}
                                    />
                                }
                            />
                        </StatsRow>
                    </StatsCard>
                    <div className={st.titleSectionContainer}>
                        <span className={st.sectionTitle}>
                            Currently Staking
                        </span>
                        <Button size="small" mode="primary">
                            <span>Claim All Rewards</span>
                            <Icon
                                icon={HaneulIcons.ArrowRight}
                                className={st.arrowIcon}
                            />
                        </Button>
                    </div>
                    <div className={st.stakedContainer}>{DEMO_STAKED}</div>
                </Content>
                <Menu stuckClass={st.shadow}>
                    <Button size="large" mode="neutral" className={st.action}>
                        <Icon
                            icon={HaneulIcons.Close}
                            className={st.closeActionIcon}
                        />
                        Cancel
                    </Button>
                    <Button size="large" mode="primary" className={st.action}>
                        Stake Coins
                        <Icon
                            icon={HaneulIcons.ArrowRight}
                            className={st.arrowActionIcon}
                        />
                    </Button>
                </Menu>
            </BottomMenuLayout>
        </div>
    );
}

export default StakeHome;
