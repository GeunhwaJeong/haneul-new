// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulClient } from '@haneullabs/haneul.js/client';
import { screen } from '@testing-library/dom';
import { render } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useState } from 'react';

import { HaneulClientProvider } from '../../src/components/HaneulClientProvider.js';
import { useHaneulClient, useHaneulClientContext } from '../../src/index.js';

describe('HaneulClientProvider', () => {
	it('renders without crashing', () => {
		render(
			<HaneulClientProvider>
				<div>Test</div>
			</HaneulClientProvider>,
		);
		expect(screen.getByText('Test')).toBeInTheDocument();
	});

	it('provides a HaneulClient instance to its children', () => {
		const ChildComponent = () => {
			const client = useHaneulClient();
			expect(client).toBeInstanceOf(HaneulClient);
			return <div>Test</div>;
		};

		render(
			<HaneulClientProvider>
				<ChildComponent />
			</HaneulClientProvider>,
		);
	});

	it('can accept pre-configured HaneulClients', () => {
		const haneulClient = new HaneulClient({ url: 'http://localhost:8080' });
		const ChildComponent = () => {
			const client = useHaneulClient();
			expect(client).toBeInstanceOf(HaneulClient);
			expect(client).toBe(haneulClient);
			return <div>Test</div>;
		};

		render(
			<HaneulClientProvider networks={{ localnet: haneulClient }}>
				<ChildComponent />
			</HaneulClientProvider>,
		);

		expect(screen.getByText('Test')).toBeInTheDocument();
	});

	test('can create haneul clients with custom options', async () => {
		function NetworkSelector() {
			const ctx = useHaneulClientContext();

			return (
				<div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => ctx.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}
		function CustomConfigProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<string>();

			return (
				<HaneulClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					createClient={(name, { custom, ...config }) => {
						custom(name);
						return new HaneulClient(config);
					}}
				>
					<div>{`selected network: ${selectedNetwork}`}</div>
					<NetworkSelector />
				</HaneulClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<CustomConfigProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});

	test('controlled mode', async () => {
		function NetworkSelector(props: { selectNetwork: (network: string) => void }) {
			const ctx = useHaneulClientContext();

			return (
				<div>
					<div>{`selected network: ${ctx.network}`}</div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => props.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}

		function ControlledProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<'a' | 'b'>('a');

			return (
				<HaneulClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					network={selectedNetwork}
				>
					<NetworkSelector
						selectNetwork={(network) => {
							setSelectedNetwork(network as 'a' | 'b');
						}}
					/>
				</HaneulClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<ControlledProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});

	test('onNetworkChange', async () => {
		function NetworkSelector() {
			const ctx = useHaneulClientContext();

			return (
				<div>
					<div>{`selected network: ${ctx.network}`}</div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => ctx.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}

		function ControlledProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<string>('a');

			return (
				<HaneulClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					network={selectedNetwork as 'a' | 'b'}
					onNetworkChange={(network) => {
						setSelectedNetwork(network);
					}}
				>
					<NetworkSelector />
				</HaneulClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<ControlledProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});
});
