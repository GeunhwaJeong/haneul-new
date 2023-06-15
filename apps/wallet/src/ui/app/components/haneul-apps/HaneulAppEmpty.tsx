// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';

import { type DisplayType } from './HaneulApp';

import st from './HaneulApp.module.scss';

export type HaneulAppEmptyProps = {
	displayType: DisplayType;
};

export function HaneulAppEmpty({ displayType }: HaneulAppEmptyProps) {
	return (
		<div className={cl(st.haneulApp, st.haneulAppEmpty, st[displayType])}>
			<div className={st.icon}></div>
			<div className={st.info}>
				<div className={st.boxOne}></div>
				{displayType === 'full' && (
					<>
						<div className={st.boxTwo}></div>
						<div className={st.boxThree}></div>
					</>
				)}
			</div>
		</div>
	);
}
