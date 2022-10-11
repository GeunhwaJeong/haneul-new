// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cn from 'classnames';

import PageTitle from '_app/shared/page-title';
import Icon, { HaneulIcons } from '_components/icon';

import type { ReactNode } from 'react';

import st from './CardLayout.module.scss';

export type CardLayoutProps = {
    title?: string;
    subtitle?: string;
    headerCaption?: string;
    icon?: 'success' | 'haneul';
    children: ReactNode | ReactNode[];
    className?: string;
    mode?: 'box' | 'plain';
    goBackOnClick?: () => void;
};

export default function CardLayout({
    className,
    children,
    title,
    subtitle,
    headerCaption,
    icon,
    mode = 'box',
    goBackOnClick,
}: CardLayoutProps) {
    return (
        <div className={cn(className, st.container, st[mode])}>
            {goBackOnClick ? (
                <PageTitle
                    onClick={goBackOnClick}
                    hideBackLabel={true}
                    className={st.back}
                />
            ) : null}
            {icon === 'success' ? (
                <div className={st.successIcon}>
                    <div className={st.successBg}>
                        <Icon
                            icon={HaneulIcons.ThumbsUp}
                            className={st.thumbsUp}
                        />
                    </div>
                </div>
            ) : null}
            {icon === 'haneul' ? (
                <div className={st.haneulIconContainer}>
                    <Icon icon={HaneulIcons.HaneulLogoIcon} className={st.haneulIcon} />
                </div>
            ) : null}
            {headerCaption ? (
                <h3 className={st.caption}>{headerCaption}</h3>
            ) : null}
            {title ? <h1 className={st.headerTitle}>{title}</h1> : null}
            {subtitle ? <h1 className={st.subTitle}>{subtitle}</h1> : null}
            {children}
        </div>
    );
}
