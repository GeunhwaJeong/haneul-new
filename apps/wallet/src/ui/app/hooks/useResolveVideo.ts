// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    type HaneulObjectResponse,
    getObjectDisplay,
    getObjectType,
} from '@haneullabs/haneul.js';

import { useRecognizedPackages } from './useRecognizedPackages';

export function useResolveVideo(object?: HaneulObjectResponse | null) {
    const recognizedPackages = useRecognizedPackages();

    if (!object) return null;

    const objectType = getObjectType(object);
    const isRecognized =
        objectType && recognizedPackages.includes(objectType.split('::')[0]);

    if (!isRecognized) return null;

    const display = getObjectDisplay(object)?.data;

    return display?.video_url;
}
