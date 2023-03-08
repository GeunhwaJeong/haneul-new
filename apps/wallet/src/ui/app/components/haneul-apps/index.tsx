// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFeature } from '@growthbook/growthbook-react';
import cl from 'classnames';
import { useMemo } from 'react';

import { useExplorerLink } from '../../hooks/useExplorerLink';
import { permissionsSelectors } from '../../redux/slices/permissions';
import { HaneulApp, type DAppEntry } from './HaneulApp';
import { HaneulAppEmpty } from './HaneulAppEmpty';
import { Button } from '_app/shared/ButtonUI';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';
import { useAppSelector } from '_hooks';
import { FEATURES } from '_src/shared/experimentation/features';
import { trackEvent } from '_src/shared/plausible';
import { prepareLinkToCompare } from '_src/shared/utils';

import st from './Playground.module.scss';

function AppsPlayGround() {
    const ecosystemApps =
        useFeature<DAppEntry[]>(FEATURES.WALLET_DAPPS).value ?? [];
    const allPermissions = useAppSelector(permissionsSelectors.selectAll);
    const linkToPermissionID = useMemo(() => {
        const map = new Map<string, string>();
        for (const aPermission of allPermissions) {
            map.set(prepareLinkToCompare(aPermission.origin), aPermission.id);
            if (aPermission.pagelink) {
                map.set(
                    prepareLinkToCompare(aPermission.pagelink),
                    aPermission.id
                );
            }
        }
        return map;
    }, [allPermissions]);
    const accountOnExplorerHref = useExplorerLink({
        type: ExplorerLinkType.address,
        useActiveAddress: true,
    });
    return (
        <div className={cl(st.container)}>
            <h4 className={st.activeSectionTitle}>Playground</h4>
            <div className={st.groupButtons}>
                <Button
                    size="tall"
                    variant="outline"
                    href={accountOnExplorerHref!}
                    text="View account on Haneul Explorer"
                    onClick={() => {
                        trackEvent('ViewExplorerAccount');
                    }}
                />
            </div>
            <div className={st.desc}>
                <div className={st.title}>Builders in haneul ecosystem</div>
                {ecosystemApps?.length ? (
                    <>
                        Apps here are actively curated but do not indicate any
                        endorsement or relationship with Haneul Wallet. Please
                        DYOR.
                    </>
                ) : null}
            </div>
            {ecosystemApps?.length ? (
                <div className={st.apps}>
                    {ecosystemApps.map((app) => (
                        <HaneulApp
                            key={app.link}
                            {...app}
                            permissionID={linkToPermissionID.get(
                                prepareLinkToCompare(app.link)
                            )}
                            displayType="full"
                        />
                    ))}
                </div>
            ) : (
                <HaneulAppEmpty displayType="full" />
            )}
        </div>
    );
}

export default AppsPlayGround;
export { default as ConnectedAppsCard } from './ConnectedAppsCard';
