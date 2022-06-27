// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { WindowMessageStream } from '_messaging/WindowMessageStream';

export function setupMessagesProxy() {
    const windowMsgStream = new WindowMessageStream(
        'haneul_content-script',
        'haneul_in-page'
    );
    windowMsgStream.messages.subscribe((msg) => {
        // TODO implement
        // eslint-disable-next-line no-console
        console.log('[ContentScriptProxy] message from inPage', msg);
    });
}
