// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { PortStream } from '_messaging/PortStream';
import { WindowMessageStream } from '_messaging/WindowMessageStream';

function createPort(windowMsgStream: WindowMessageStream) {
    const port = PortStream.connectToBackgroundService(
        'haneul_content<->background'
    );
    port.onMessage.subscribe((msg) => {
        windowMsgStream.send(msg);
    });
    const windowMsgSub = windowMsgStream.messages.subscribe((msg) => {
        port.sendMessage(msg);
    });
    port.onDisconnect.subscribe((port) => {
        windowMsgSub.unsubscribe();
        createPort(windowMsgStream);
    });
}

export function setupMessagesProxy() {
    const windowMsgStream = new WindowMessageStream(
        'haneul_content-script',
        'haneul_in-page'
    );
    createPort(windowMsgStream);
}
