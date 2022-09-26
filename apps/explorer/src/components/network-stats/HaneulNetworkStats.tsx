// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { IS_STATIC_ENV } from '../../utils/envUtil';

import styles from './HaneulNetworkStats.module.css';

//TODO add the backend service to get all Network stats data
function HaneulNetworkCard({ count }: { count: number | string }) {
    const totalStatsData = [
        {
            title: 'TOTAL Objects',
            value: '372.5M',
        },
        {
            title: 'TOTAL MODULES',
            value: '153,510',
        },
        {
            title: 'TOTAL BYTES STORED',
            value: '2.591B',
        },
        {
            title: 'TOTAL TRANSACTIONS',
            value: count,
        },
    ];

    const currentStatsData = [
        {
            title: 'CURRENT HANEUL PRICE',
            value: '$26.45',
        },
        {
            title: 'Current Epoch',
            value: '142,215',
        },
        {
            title: 'CURRENT VALIDATORS',
            value: '15,482',
        },
        {
            title: 'CURRENT TPS',
            value: '2,125',
        },
    ];

    return (
        <div className={styles.networkstats}>
            <div className={styles.statsitems}>
                {totalStatsData.map((item, idx) => (
                    <div className={styles.statsitem} key={idx}>
                        {item.title}
                        <span className={styles.stats}>{item.value}</span>
                    </div>
                ))}
            </div>
            <div className={styles.statsitems}>
                {currentStatsData.map((item, idx) => (
                    <div className={styles.statsitem} key={idx}>
                        {item.title}
                        <span className={styles.stats}>{item.value}</span>
                    </div>
                ))}
            </div>
        </div>
    );
}

function HaneulNetworkCardStatic() {
    return <HaneulNetworkCard count={3030} />;
}

const HaneulNetworkStats = ({ count }: { count: number }) =>
    IS_STATIC_ENV ? <HaneulNetworkCardStatic /> : <HaneulNetworkCard count={count} />;

export default HaneulNetworkStats;
