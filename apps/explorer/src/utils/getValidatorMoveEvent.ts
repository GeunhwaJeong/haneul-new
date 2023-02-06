// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulEventEnvelope, type MoveEvent } from '@haneullabs/haneul.js';

export function getValidatorMoveEvent(
    validatorsEvent: HaneulEventEnvelope[],
    validatorAddress: string
) {
    const event = validatorsEvent.find(({ event }) => {
        if ('moveEvent' in event) {
            const { moveEvent } = event as { moveEvent: MoveEvent };
            return moveEvent.fields.validator_address === validatorAddress;
        }
        return false;
    });

    return event && 'moveEvent' in event.event ? event.event.moveEvent : null;
}
