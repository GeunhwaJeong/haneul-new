// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import Plausible from 'plausible-tracker';

export const ToS_LINK = 'https://haneul.io/terms';
export const PRIVACY_POLICY_LINK = 'https://haneul.io/policy/';
// NOTE: The url of Haneul wallet Chrome extension:
// https://chrome.google.com/webstore/detail/haneul-wallet/opcgpfmipidbgpenhmajoajpbobppdil
export const WALLET_URL = 'chrome-extension://opcgpfmipidbgpenhmajoajpbobppdil';
export const plausible = Plausible({
    domain: WALLET_URL,
});
