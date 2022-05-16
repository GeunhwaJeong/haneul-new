// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import Loading from '_components/loading';
import { useInitializedGuard } from '_hooks';
import logo from '_images/haneul-icon.png';

import st from './Home.module.scss';

const HomePage = () => {
    const guardChecking = useInitializedGuard(true);
    return (
        <Loading loading={guardChecking}>
            <div className={st.container}>
                <img className={st.logo} src={logo} alt="logo" />
                <h2>Under Construction</h2>
            </div>
        </Loading>
    );
};

export default HomePage;
