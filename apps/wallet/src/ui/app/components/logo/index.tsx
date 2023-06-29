// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulMainnet, HaneulTestnet, HaneulDevnet, HaneulLocal, HaneulCustomRpc } from '@haneullabs/icons';

import { API_ENV } from '_src/shared/api-env';

type LogoProps = {
	networkName?: API_ENV;
};

const networkLogos = {
	[API_ENV.mainnet]: HaneulMainnet,
	[API_ENV.devNet]: HaneulDevnet,
	[API_ENV.testNet]: HaneulTestnet,
	[API_ENV.local]: HaneulLocal,
	[API_ENV.customRPC]: HaneulCustomRpc,
};

const Logo = ({ networkName }: LogoProps) => {
	const LogoComponent = networkName ? networkLogos[networkName] : networkLogos[API_ENV.mainnet];

	return <LogoComponent className="h-7 w-walletLogo" />;
};

export default Logo;
