// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Link } from 'react-router-dom';

import styles from './Header.module.css';

const Header = () => {
    return (
        <header>
            <nav className={styles.nav}>
                <Link to="/" aria-label="logo" className={styles.logo}>
                    Haneul Labs
                </Link>
            </nav>
        </header>
    );
};

export default Header;
