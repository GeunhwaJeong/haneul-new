// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Link } from 'react-router-dom';

import ExternalLink from '../external-link/ExternalLink';

import styles from './Footer.module.css';

function Footer() {
    return (
        <footer className={styles.footer}>
            <nav className={styles.links}>
                <Link to="/" id="homeBtn">
                    Home
                </Link>
                <ExternalLink href="https://haneul.io/" label="Haneul" />
                <ExternalLink
                    href="https://haneul-labs.com/"
                    label="Haneul Labs"
                />
                <ExternalLink
                    href="https://docs.haneul.io/"
                    label="Developer Hub"
                />
            </nav>
        </footer>
    );
}

export default Footer;
