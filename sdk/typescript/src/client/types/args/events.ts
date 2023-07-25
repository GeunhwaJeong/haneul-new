// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulJsonValue } from '../common.js';

export type MoveEventField = {
	path: string;
	value: HaneulJsonValue;
};

// mirrors haneul_json_rpc_types::HaneulEventFilter
export type HaneulEventFilter =
	| { Package: string }
	| { MoveModule: { package: string; module: string } }
	| { MoveEventType: string }
	| { MoveEventField: MoveEventField }
	| { Transaction: string }
	| {
			TimeRange: {
				// left endpoint of time interval, milliseconds since epoch, inclusive
				startTime: string;
				// right endpoint of time interval, milliseconds since epoch, exclusive
				endTime: string;
			};
	  }
	| { Sender: string }
	| { All: HaneulEventFilter[] }
	| { Any: HaneulEventFilter[] }
	| { And: [HaneulEventFilter, HaneulEventFilter] }
	| { Or: [HaneulEventFilter, HaneulEventFilter] };
