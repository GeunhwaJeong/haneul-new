// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import TransportWebHID from '@ledgerhq/hw-transport-webhid';
import TransportWebUSB from '@ledgerhq/hw-transport-webusb';
import HaneulLedgerClient from '@haneullabs/ledgerjs-hw-app-haneul';
import {
    createContext,
    useCallback,
    useContext,
    useEffect,
    useMemo,
    useState,
} from 'react';

import {
    LedgerConnectionFailedError,
    LedgerNoTransportMechanismError,
} from './ledgerErrors';

type HaneulLedgerClientProviderProps = {
    children: React.ReactNode;
};

type HaneulLedgerClientContextValue = {
    haneulLedgerClient: HaneulLedgerClient | undefined;
    connectToLedger: () => Promise<HaneulLedgerClient>;
};

const HaneulLedgerClientContext = createContext<
    HaneulLedgerClientContextValue | undefined
>(undefined);

export function HaneulLedgerClientProvider({
    children,
}: HaneulLedgerClientProviderProps) {
    const [haneulLedgerClient, setHaneulLedgerClient] = useState<HaneulLedgerClient>();

    useEffect(() => {
        const onDisconnect = () => {
            setHaneulLedgerClient(undefined);
        };

        haneulLedgerClient?.transport.on('disconnect', onDisconnect);
        return () => haneulLedgerClient?.transport.off('disconnect', onDisconnect);
    }, [haneulLedgerClient?.transport]);

    const connectToLedger = useCallback(async () => {
        if (haneulLedgerClient?.transport) {
            // If we've already connected to a Ledger device, we need
            // to close the connection before we try to re-connect
            await haneulLedgerClient.transport.close();
        }

        const ledgerTransport = await requestConnectionToLedger();
        const ledgerClient = new HaneulLedgerClient(ledgerTransport);
        setHaneulLedgerClient(ledgerClient);
        return ledgerClient;
    }, [haneulLedgerClient]);

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
        throw new Error(
            'useHaneulLedgerClient use must be within HaneulLedgerClientContext'
        );
    }
    return haneulLedgerClientContext;
}

async function requestConnectionToLedger() {
    try {
        if (await TransportWebHID.isSupported()) {
            return await TransportWebHID.request();
        } else if (await TransportWebUSB.isSupported()) {
            return await TransportWebUSB.request();
        }
    } catch (error) {
        throw new LedgerConnectionFailedError(
            "Unable to connect to the user's Ledger device"
        );
    }
    throw new LedgerNoTransportMechanismError(
        "There are no supported transport mechanisms to connect to the user's Ledger device"
    );
}
