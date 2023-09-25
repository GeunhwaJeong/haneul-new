// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { API_ENV } from '_src/shared/api-env';
import { HaneulCustomRpc, HaneulDevnet, HaneulLocal, HaneulMainnet, HaneulTestnet } from '@haneullabs/icons';

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

	return <LogoComponent className="h-7 w-walletLogo text-gray-90" />;
};

export default Logo;
