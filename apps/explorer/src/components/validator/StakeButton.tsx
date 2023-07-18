// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { Button } from '@haneullabs/ui';
import {
	ConnectButton,
	useWalletKit,
	type StandardWalletAdapter,
	type WalletWithFeatures,
} from '@haneullabs/wallet-kit';
import { useParams } from 'react-router-dom';

// This is a custom feature supported by the Haneul Wallet:
type StakeInput = { validatorAddress: string };
type HaneulWalletStakeFeature = {
	'haneulWallet:stake': {
		version: '0.0.1';
		stake: (input: StakeInput) => Promise<void>;
	};
};

type StakeWallet = WalletWithFeatures<Partial<HaneulWalletStakeFeature>>;

export function StakeButton() {
	const stakeButtonEnabled = useFeatureIsOn('validator-page-staking');
	const { id } = useParams();
	const { wallets, currentWallet, connect } = useWalletKit();

	if (!stakeButtonEnabled) return null;

	const stakeSupportedWallets = wallets.filter((wallet) => {
		if (!('wallet' in wallet)) {
			return false;
		}

		const standardWallet = wallet.wallet as StakeWallet;
		return 'haneulWallet:stake' in standardWallet.features;
	});

	const currentWalletSupportsStake =
		currentWallet && !!stakeSupportedWallets.find(({ name }) => currentWallet.name === name);

	if (!stakeSupportedWallets.length) {
		return (
			<Button size="lg" asChild>
				<a
					href="https://chrome.google.com/webstore/detail/haneul-wallet/opcgpfmipidbgpenhmajoajpbobppdil"
					target="_blank"
					rel="noreferrer noopener"
				>
					Install Haneul Wallet to stake HANEUL
				</a>
			</Button>
		);
	}

	if (!currentWallet) {
		return (
			<ConnectButton
				className="!border !border-solid !border-steel-dark !bg-transparent !px-4 !py-3 !text-body !font-semibold !text-steel-dark !shadow-none"
				connectText="Stake HANEUL"
			/>
		);
	}

	if (!currentWalletSupportsStake) {
		return (
			<Button
				size="lg"
				onClick={() => {
					// Always just assume we should connect to the first stake supported wallet for now:
					connect(stakeSupportedWallets[0].name);
				}}
			>
				Stake HANEUL on a supported wallet
			</Button>
		);
	}

	return (
		<Button
			size="lg"
			onClick={() => {
				((currentWallet as StandardWalletAdapter).wallet as StakeWallet).features[
					'haneulWallet:stake'
				]?.stake({ validatorAddress: id! });
			}}
		>
			Stake HANEUL
		</Button>
	);
}
