// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { memo, useMemo } from 'react';

import { HaneulIcons } from '_font-icons/output/haneul-icons';

import type { MouseEventHandler } from 'react';

export { HaneulIcons } from '_font-icons/output/haneul-icons';

export type IconProps = {
    className?: string;
    icon: HaneulIcons | string;
    onClick?: MouseEventHandler<HTMLElement>;
    title?: string;
};

const isHaneulIconMap: Record<string, boolean> = Object.values(HaneulIcons).reduce<
    Record<string, boolean>
>((acc, anIcon) => {
    acc[anIcon] = true;
    return acc;
}, {});

function Icon({ className, icon, onClick, title }: IconProps) {
    const isHaneulIcon = useMemo(() => isHaneulIconMap[icon] || false, [icon]);
    return (
        <i
            className={cl(className, {
                [`bi-${icon}`]: !isHaneulIcon,
                bi: !isHaneulIcon,
                [icon]: isHaneulIcon,
            })}
            onClick={onClick}
            title={title}
        ></i>
    );
}

export default memo(Icon);
