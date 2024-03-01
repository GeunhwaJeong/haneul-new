// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { matchRoutes, useLocation, useParams } from 'react-router-dom';
import { useNetworkContext } from '~/context';
import { Network } from '~/utils/api/DefaultRpcClient';
import { useMemo } from 'react';
import { useGetObject } from '../../../core';
import { translate } from '~/pages/object-result/ObjectResultType';

const HANEULSCAN_URL_MAINNET = 'https://haneulscan.xyz';
const HANEULSCAN_URL_TESTNET = 'https://haneulscan.xyz/testnet';
const HANEULSCAN_URL_DEVNET = 'https://haneulscan.xyz/devnet';
const HANEULVISION_URL_MAINNET = 'https://haneulvision.xyz';
const HANEULVISION_URL_TESTNET = 'https://testnet.haneulvision.xyz';
const HANEULVISION_URL_DEVNET = 'https://haneulvision.xyz';

enum Routes {
	object = '/object/:id',
	checkpoint = '/checkpoint/:id',
	txblock = '/txblock/:id',
	epoch = '/epoch/:id',
	address = '/address/:id',
	validator = '/validator/:id',
	validators = '/validators',
}

function useMatchPath() {
	const location = useLocation();
	const someRoutes = [
		{ path: Routes.object },
		{ path: Routes.checkpoint },
		{ path: Routes.txblock },
		{ path: Routes.epoch },
		{ path: Routes.address },
		{ path: Routes.validator },
		{ path: Routes.validators },
	];
	const matches = matchRoutes(someRoutes, location);
	return matches?.[0]?.route.path;
}

export function useRedirectUrl(isPackage?: boolean) {
	const [network] = useNetworkContext();
	const { id } = useParams();

	const matchPath = useMatchPath();
	const hasMatch = Boolean(matchPath);

	const baseUrl = useMemo(() => {
		switch (network) {
			case Network.DEVNET:
				return {
					haneulscan: HANEULSCAN_URL_DEVNET,
					haneulvision: HANEULVISION_URL_DEVNET,
				};
			case Network.TESTNET:
				return {
					haneulscan: HANEULSCAN_URL_TESTNET,
					haneulvision: HANEULVISION_URL_TESTNET,
				};
			default:
				return {
					haneulscan: HANEULSCAN_URL_MAINNET,
					haneulvision: HANEULVISION_URL_MAINNET,
				};
		}
	}, [network]);

	const redirectPathname = useMemo(() => {
		switch (matchPath) {
			case Routes.object:
				return {
					haneulscan: `/object/${id}`,
					haneulvision: isPackage ? `/package/${id}` : `/object/${id}`,
				};
			case Routes.checkpoint:
				return {
					haneulscan: `/checkpoint/${id}`,
					haneulvision: `/checkpoint/${id}`,
				};
			case Routes.txblock:
				return {
					haneulscan: `/tx/${id}`,
					haneulvision: `/txblock/${id}`,
				};
			case Routes.epoch:
				return {
					haneulscan: `/epoch/${id}`,
					haneulvision: `/epoch/${id}`,
				};
			case Routes.address:
				return {
					haneulscan: `/address/${id}`,
					haneulvision: `/address/${id}`,
				};
			case Routes.validator:
				return {
					haneulscan: `/validator/${id}`,
					haneulvision: `/validator/${id}`,
				};
			case Routes.validators:
				return {
					haneulscan: `/validators`,
					haneulvision: `/validators`,
				};
			default: {
				return {
					haneulscan: '/',
					haneulvision: '/',
				};
			}
		}
	}, [id, matchPath, isPackage]);

	return {
		haneulvisionUrl: `${baseUrl.haneulvision}${redirectPathname.haneulvision}`,
		haneulscanUrl: `${baseUrl.haneulscan}${redirectPathname.haneulscan}`,
		hasMatch,
	};
}

function useRedirectObject() {
	const { id } = useParams();
	const { data, isError } = useGetObject(id);
	const resp = data && !isError ? translate(data) : null;
	const isPackage = resp ? resp.objType === 'Move Package' : false;

	return useRedirectUrl(isPackage);
}

export function useRedirectExplorerUrl() {
	const matchPath = useMatchPath();
	const useRedirectHook = matchPath === Routes.object ? useRedirectObject : useRedirectUrl;
	return useRedirectHook();
}
