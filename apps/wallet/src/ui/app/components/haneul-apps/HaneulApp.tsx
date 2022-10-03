// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { memo, useState, useCallback } from 'react';

import DisconnectApp from './DisconnectApp';
import ExternalLink from '_components/external-link';
import Icon, { HaneulIcons } from '_components/icon';
import { trackEvent } from '_src/shared/plausible';

import st from './HaneulApp.module.scss';

type Displaytype = {
    displaytype: 'full' | 'card';
};

type HaneulAppProps = {
    name?: string;
    description?: string;
    icon?: string;
    displaytype: 'full' | 'card';
    tags?: string[];
    link: string;
    account?: string;
    id?: string;
    permissions: string[];
    disconnect?: boolean;
};

function HaneulAppEmpty({ displaytype }: Displaytype) {
    return (
        <div className={cl(st.haneulApp, st.haneulAppEmpty, st[displaytype])}>
            <div className={st.icon}></div>
            <div className={st.info}>
                <div className={st.boxOne}></div>
                {displaytype === 'full' && (
                    <>
                        <div className={st.boxTwo}></div>
                        <div className={st.boxThree}></div>
                    </>
                )}
            </div>
        </div>
    );
}

function HaneulApp({
    name,
    description,
    icon,
    displaytype,
    link,
    tags,
    id,
    account,
    permissions,
    disconnect,
}: HaneulAppProps) {
    const [showDisconnectApp, setShowDisconnectApp] = useState(false);
    const appData = {
        name: name || 'Unknown App',
        icon,
        link,
        id,
        permissions,
    };
    const AppDetails = (
        <div className={cl(st.haneulApp, st[displaytype])}>
            <div className={st.icon}>
                {icon ? (
                    <img src={icon} className={st.icon} alt={name} />
                ) : (
                    <div className={st.defaultImg}></div>
                )}
                {displaytype === 'card' && (
                    <Icon
                        icon={HaneulIcons.ArrowRight}
                        className={cl(
                            st.arrowActionIcon,
                            st.angledArrow,
                            st.externalLinkIcon
                        )}
                    />
                )}
            </div>
            <div className={st.info}>
                <div className={st.title}>
                    {name}{' '}
                    {displaytype === 'full' && (
                        <Icon
                            icon={HaneulIcons.ArrowRight}
                            className={cl(st.arrowActionIcon, st.angledArrow)}
                        />
                    )}
                </div>
                {displaytype === 'full' && (
                    <div className={st.description}>{description}</div>
                )}

                {displaytype === 'card' && (
                    <div className={st.link}>{link}</div>
                )}

                {displaytype === 'full' && tags?.length && (
                    <div className={st.tags}>
                        {tags?.map((tag) => (
                            <div className={st.tag} key={tag}>
                                {tag}
                            </div>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );

    const openApp = useCallback(
        (e: React.MouseEvent<HTMLElement>) => {
            setShowDisconnectApp(true);
        },
        [setShowDisconnectApp]
    );

    const onClickAppLink = useCallback(() => {
        trackEvent('AppOpen', {
            props: { name: name || link, source: 'AppPage' },
        });
    }, [name, link]);

    return (
        <>
            {showDisconnectApp && (
                <DisconnectApp
                    {...appData}
                    setShowDisconnectApp={setShowDisconnectApp}
                />
            )}
            {disconnect ? (
                <>
                    <div className={st.ecosystemApp} onClick={openApp}>
                        {AppDetails}
                    </div>
                </>
            ) : (
                <ExternalLink
                    href={link}
                    title={name}
                    className={st.ecosystemApp}
                    showIcon={false}
                    onClick={onClickAppLink}
                >
                    {AppDetails}
                </ExternalLink>
            )}
        </>
    );
}

export default memo(HaneulApp);
export { HaneulAppEmpty };
