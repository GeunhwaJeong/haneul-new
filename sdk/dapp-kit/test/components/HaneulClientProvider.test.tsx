// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { render } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { screen } from '@testing-library/dom';
import { HaneulClientProvider } from '../../src/components/HaneulClientProvider.js';
import { useHaneulClient, useHaneulClientContext } from 'dapp-kit/src/index.js';
import { HaneulClient } from '@haneullabs/haneul.js/client';
import { useState } from 'react';

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
});
