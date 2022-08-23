// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { Link } from 'react-router-dom';

import BottomMenuLayout, { Content } from '_app/shared/bottom-menu-layout';
import Icon, { HaneulIcons } from '_components/icon';
import Loading from '_components/loading';
import Logo from '_components/logo';
import { useInitializedGuard } from '_hooks';
import PageLayout from '_pages/layout';

import st from './Welcome.module.scss';

const WelcomePage = () => {
    const checkingInitialized = useInitializedGuard(false);
    return (
        <PageLayout forceFullscreen={true}>
            <Loading loading={checkingInitialized}>
                <div className={st.container}>
                    <div className={st.content}>
                        <BottomMenuLayout>
                            <Content className={st.welcome}>
                                <Logo
                                    size="bigger"
                                    className={st.haneulBlue}
                                    txt={true}
                                />

                                <div className={st.description}>
                                    <h1 className={st.title}>
                                        Welcome to Haneul Wallet
                                    </h1>
                                    <p>
                                        Connecting you to the decentralized web
                                        and HANEUL network.
                                    </p>
                                    <ul className={st.features}>
                                        <li>
                                            <Icon icon={HaneulIcons.Checkmark} />
                                            Buy, store, send and swap tokens
                                        </li>
                                        <li>
                                            <Icon icon={HaneulIcons.Checkmark} />
                                            Explore blockchain apps
                                        </li>
                                        <li>
                                            <Icon icon={HaneulIcons.Checkmark} />
                                            Find the best price every time
                                        </li>
                                    </ul>
                                </div>
                            </Content>
                            <div className={st.getStarted}>
                                <Link
                                    to="/initialize/select"
                                    className={cl(st.cta, 'btn', 'primary')}
                                >
                                    Get Started
                                    <Icon
                                        icon={HaneulIcons.ArrowLeft}
                                        className={cl(st.arrowLeft)}
                                    />
                                </Link>
                            </div>
                        </BottomMenuLayout>
                    </div>
                </div>
            </Loading>
        </PageLayout>
    );
};

export default WelcomePage;
