// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const HANEUL_NS_NAME_REGEX =
	/^(?!.*(^(?!@)|[-.@])($|[-.@]))(?:[a-z0-9-]{0,63}(?:\.[a-z0-9-]{0,63})*)?@[a-z0-9-]{0,63}$/i;
const HANEUL_NS_DOMAIN_REGEX = /^(?!.*(^|[-.])($|[-.]))(?:[a-z0-9-]{0,63}\.)+haneul$/i;
const MAX_HANEUL_NS_NAME_LENGTH = 235;

export function isValidHaneulNSName(name: string): boolean {
	if (name.length > MAX_HANEUL_NS_NAME_LENGTH) {
		return false;
	}

	if (name.includes('@')) {
		return HANEUL_NS_NAME_REGEX.test(name);
	}

	return HANEUL_NS_DOMAIN_REGEX.test(name);
}

export function normalizeHaneulNSName(name: string, format: 'at' | 'dot' = 'at'): string {
	const lowerCase = name.toLowerCase();
	let parts;

	if (lowerCase.includes('@')) {
		if (!HANEUL_NS_NAME_REGEX.test(lowerCase)) {
			throw new Error(`Invalid HaneulNS name ${name}`);
		}
		const [labels, domain] = lowerCase.split('@');
		parts = [...(labels ? labels.split('.') : []), domain];
	} else {
		if (!HANEUL_NS_DOMAIN_REGEX.test(lowerCase)) {
			throw new Error(`Invalid HaneulNS name ${name}`);
		}
		parts = lowerCase.split('.').slice(0, -1);
	}

	if (format === 'dot') {
		return `${parts.join('.')}.haneul`;
	}

	return `${parts.slice(0, -1).join('.')}@${parts[parts.length - 1]}`;
}
