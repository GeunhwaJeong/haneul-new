// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { formatAmount } from '@haneullabs/core';
import { Haneul } from '@haneullabs/icons';

import { StatsWrapper } from './HomeMetrics/FormattedStatsAmount';

import { useHaneulCoinData } from '~/hooks/useHaneulCoinData';
import { Card } from '~/ui/Card';
import { Heading } from '~/ui/Heading';
import { Text } from '~/ui/Text';

export function HaneulTokenCard() {
    const { data, isLoading } = useHaneulCoinData();
    const {
        // priceChangePercentageOver24H,
        currentPrice,
        totalSupply,
        fullyDilutedMarketCap,
    } = data || {};

    // const isPriceChangePositive = Number(priceChangePercentageOver24H) > 0;
    const formattedPrice = currentPrice
        ? currentPrice.toLocaleString('en', {
              style: 'currency',
              currency: 'USD',
          })
        : '--';

    return (
        <Card bg="lightBlue" spacing="lg">
            <div className="md:max-lg:max-w-[336px]">
                <div className="flex items-center gap-2">
                    <div className="h-4.5 w-4.5 rounded-full bg-haneul p-1">
                        <Haneul className="h-full w-full text-white" />
                    </div>
                    <div className="flex w-full flex-col gap-1 sm:flex-row sm:items-center sm:justify-between">
                        <div className="flex items-center gap-2">
                            <Heading
                                as="div"
                                variant="heading4/semibold"
                                color="steel-darker"
                            >
                                1 HANEUL = {formattedPrice}
                            </Heading>
                            {/* {priceChangePercentageOver24H ? (
                                <Heading
                                    as="div"
                                    variant="heading6/medium"
                                    color={
                                        isPriceChangePositive
                                            ? 'success'
                                            : 'issue'
                                    }
                                >
                                    {isPriceChangePositive ? '+' : null}
                                    {priceChangePercentageOver24H.toFixed(2)}%
                                </Heading>
                            ) : null} */}
                        </div>
                        <Text variant="subtitleSmallExtra/medium" color="steel">
                            via CoinGecko
                        </Text>
                    </div>
                </div>
                <div className="mt-8 flex w-full gap-8">
                    <StatsWrapper
                        label="Market Cap"
                        size="sm"
                        postfix="USD"
                        unavailable={isLoading}
                    >
                        {formatAmount(fullyDilutedMarketCap)}
                    </StatsWrapper>
                    <StatsWrapper
                        label="Total Supply"
                        size="sm"
                        postfix="HANEUL"
                        unavailable={isLoading}
                    >
                        {formatAmount(totalSupply)}
                    </StatsWrapper>
                </div>
            </div>
        </Card>
    );
}
