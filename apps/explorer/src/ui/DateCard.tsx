// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { formatDate } from '@haneullabs/core';
import { Text } from '@haneullabs/ui';

export interface DateCardProps {
	date: Date | number;
}

// TODO - add format options
export function DateCard({ date }: DateCardProps) {
	const dateStr = formatDate(date, ['month', 'day', 'year', 'hour', 'minute']);

	if (!dateStr) {
		return null;
	}

	return (
		<Text variant="bodySmall/semibold" color="steel-dark">
			<time dateTime={new Date(date).toISOString()}>{dateStr}</time>
		</Text>
	);
}
