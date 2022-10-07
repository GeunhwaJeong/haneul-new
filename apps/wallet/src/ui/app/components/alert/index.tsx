// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { memo } from 'react';

import Icon, { HaneulIcons } from '_components/icon';

import type { ReactNode } from 'react';

import st from './Alert.module.scss';

export type AlertProps = {
    children: ReactNode | ReactNode[];
    className?: string;
};

function Alert({ children, className }: AlertProps) {
    return (
        <div className={cl(st.container, st.error, className)}>
            <Icon className={st.icon} icon={HaneulIcons.Info} />
            <div className={st.message}>{children}</div>
        </div>
    );
}

export default memo(Alert);
