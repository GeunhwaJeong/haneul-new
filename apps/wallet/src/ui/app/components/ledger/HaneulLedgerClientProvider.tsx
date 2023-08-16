// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import TransportWebHID from '@ledgerhq/hw-transport-webhid';
import TransportWebUSB from '@ledgerhq/hw-transport-webusb';
import HaneulLedgerClient from '@haneullabs/ledgerjs-hw-app-haneul';
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';

import {
	convertErrorToLedgerConnectionFailedError,
	LedgerDeviceNotFoundError,
	LedgerNoTransportMechanismError,
} from './ledgerErrors';

type HaneulLedgerClientProviderProps = {
	children: React.ReactNode;
};

type HaneulLedgerClientContextValue = {
	haneulLedgerClient: HaneulLedgerClient | undefined;
	connectToLedger: (requestPermissionsFirst?: boolean) => Promise<HaneulLedgerClient>;
};

const HaneulLedgerClientContext = createContext<HaneulLedgerClientContextValue | undefined>(undefined);

export function HaneulLedgerClientProvider({ children }: HaneulLedgerClientProviderProps) {
	const [haneulLedgerClient, setHaneulLedgerClient] = useState<HaneulLedgerClient>();
	const resetHaneulLedgerClient = useCallback(async () => {
		await haneulLedgerClient?.transport.close();
		setHaneulLedgerClient(undefined);
	}, [haneulLedgerClient]);

	useEffect(() => {
		// NOTE: The disconnect event is fired when someone physically disconnects
		// their Ledger device in addition to when user's exit out of an application
		haneulLedgerClient?.transport.on('disconnect', resetHaneulLedgerClient);
		return () => {
			haneulLedgerClient?.transport.off('disconnect', resetHaneulLedgerClient);
		};
	}, [resetHaneulLedgerClient, haneulLedgerClient?.transport]);

	const connectToLedger = useCallback(
		async (requestPermissionsFirst = false) => {
			// If we've already connected to a Ledger device, we need
			// to close the connection before we try to re-connect
			await resetHaneulLedgerClient();

			const ledgerTransport = requestPermissionsFirst
				? await requestLedgerConnection()
				: await openLedgerConnection();
			const ledgerClient = new HaneulLedgerClient(ledgerTransport);
			setHaneulLedgerClient(ledgerClient);
			return ledgerClient;
		},
		[resetHaneulLedgerClient],
	);
	const contextValue: HaneulLedgerClientContextValue = useMemo(() => {
		return {
			haneulLedgerClient,
			connectToLedger,
		};
	}, [connectToLedger, haneulLedgerClient]);

	return (
		<HaneulLedgerClientContext.Provider value={contextValue}>
			{children}
		</HaneulLedgerClientContext.Provider>
	);
}

export function useHaneulLedgerClient() {
	const haneulLedgerClientContext = useContext(HaneulLedgerClientContext);
	if (!haneulLedgerClientContext) {
		throw new Error('useHaneulLedgerClient must be used within HaneulLedgerClientContext');
	}
	return haneulLedgerClientContext;
}

async function requestLedgerConnection() {
	const ledgerTransportClass = await getLedgerTransportClass();
	try {
		return await ledgerTransportClass.request();
	} catch (error) {
		throw convertErrorToLedgerConnectionFailedError(error);
	}
}

async function openLedgerConnection() {
	const ledgerTransportClass = await getLedgerTransportClass();
	let ledgerTransport: TransportWebHID | TransportWebUSB | null | undefined;

	try {
		ledgerTransport = await ledgerTransportClass.openConnected();
	} catch (error) {
		throw convertErrorToLedgerConnectionFailedError(error);
	}
	if (!ledgerTransport) {
		throw new LedgerDeviceNotFoundError(
			"The user doesn't have a Ledger device connected to their machine",
		);
	}
	return ledgerTransport;
}

async function getLedgerTransportClass() {
	if (await TransportWebHID.isSupported()) {
		return TransportWebHID;
	} else if (await TransportWebUSB.isSupported()) {
		return TransportWebUSB;
	}
	throw new LedgerNoTransportMechanismError(
		"There are no supported transport mechanisms to connect to the user's Ledger device",
	);
}
