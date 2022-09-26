// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Link } from 'react-router-dom';

import { ReactComponent as HaneulLogo } from '../../assets/Haneul Logo.svg';
import NetworkSelect from '../network/Network';
import Search from '../search/Search';

import styles from './Header.module.css';

const Header = () => {
    return (
        <header>
            <Link
                id="homeBtn"
                data-testid="nav-logo-button"
                className={styles.haneultitle}
                to="/"
            >
                <HaneulLogo />
            </Link>

            <div className={styles.search}>
                <Search />
            </div>

            <div className={styles.networkselect}>
                <NetworkSelect />
            </div>
        </header>
    );
};

export default Header;
