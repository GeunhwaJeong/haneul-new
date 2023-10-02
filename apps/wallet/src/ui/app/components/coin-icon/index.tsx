// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ImageIcon } from '_app/shared/image-icon';
import { useCoinMetadata } from '@haneullabs/core';
import { Haneul, Unstaked } from '@haneullabs/icons';
import { HANEUL_TYPE_ARG } from '@haneullabs/haneul.js/utils';
import { cva, type VariantProps } from 'class-variance-authority';

const imageStyle = cva(['rounded-full flex'], {
	variants: {
		size: {
			sm: 'w-6 h-6',
			md: 'w-7.5 h-7.5',
			lg: 'md:w-10 md:h-10 w-8 h-8',
			xl: 'md:w-31.5 md:h-31.5 w-16 h-16 ',
		},
		fill: {
			haneul: 'bg-haneul',
			haneulPrimary2023: 'bg-haneul-primaryBlue2023',
		},
	},
	defaultVariants: {
		size: 'md',
		fill: 'haneulPrimary2023',
	},
});

function HaneulCoin() {
	return (
		<Haneul className="flex items-center w-full h-full justify-center text-white p-1.5 text-body rounded-full" />
	);
}

type NonHaneulCoinProps = {
	coinType: string;
};

function NonHaneulCoin({ coinType }: NonHaneulCoinProps) {
	const { data: coinMeta } = useCoinMetadata(coinType);
	return (
		<div className="flex h-full w-full items-center justify-center text-white bg-steel rounded-full">
			{coinMeta?.iconUrl ? (
				<ImageIcon
					src={coinMeta.iconUrl}
					label={coinMeta.name || coinType}
					fallback={coinMeta.name || coinType}
					rounded="full"
				/>
			) : (
				<Unstaked />
			)}
		</div>
	);
}

export interface CoinIconProps extends VariantProps<typeof imageStyle> {
	coinType: string;
}

export function CoinIcon({ coinType, ...styleProps }: CoinIconProps) {
	return (
		<div className={imageStyle(styleProps)}>
			{coinType === HANEUL_TYPE_ARG ? <HaneulCoin /> : <NonHaneulCoin coinType={coinType} />}
		</div>
	);
}
