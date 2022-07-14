// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { memo } from 'react';
import { Link } from 'react-router-dom';

import Icon, { HaneulIcons } from '_components/icon';

import type { ReactNode } from 'react';

import st from './Layout.module.scss';

export type LayoutProps = {
    backUrl?: string;
    title: string;
    children: ReactNode | ReactNode[];
};

function Layout({ backUrl, title, children }: LayoutProps) {
    return (
        <div className={st.container}>
            <div className={st.header}>
                {backUrl ? (
                    <Link to={backUrl} className={st.arrowBack}>
                        <Icon icon={HaneulIcons.ArrowLeft} />
                    </Link>
                ) : null}
                <div className={st.title}>{title}</div>
            </div>
            {children}
        </div>
    );
}

export default memo(Layout);
