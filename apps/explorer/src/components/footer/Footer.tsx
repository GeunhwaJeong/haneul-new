// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ReactComponent as HaneulLogoIcon } from '../../assets/Haneul Logo.svg';

import styles from './Footer.module.css';

import { Link } from '~/ui/Link';

function Footer() {
    return (
        <footer>
            <nav className={styles.links}>
                <div className={styles.logodesktop}>
                    <HaneulLogoIcon />
                    <div className={styles.copyright}>
                        <div>&copy;2022 Haneul</div>
                        <div>All rights reserved</div>
                    </div>
                </div>

                <div>
                    <h6>Read</h6>
                    <ul>
                        <li>
                            <Link href="https://medium.com/haneullabs-labs">
                                Blog
                            </Link>
                        </li>
                        <li>
                            <Link href="https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf">
                                Whitepaper
                            </Link>
                        </li>
                    </ul>
                </div>
                <div>
                    <h6>Build</h6>
                    <ul>
                        <li>
                            <Link href="https://docs.haneul.io/">Docs</Link>
                        </li>
                        <li>
                            <Link href="https://github.com/GeunhwaJeong">
                                GitHub
                            </Link>
                        </li>
                        <li>
                            <Link href="https://discord.gg/haneul">Discord</Link>
                        </li>
                    </ul>
                </div>
                <div>
                    <h6>Follow</h6>
                    <ul>
                        <li>
                            <Link href="https://haneul-labs.com/#community">
                                Press
                            </Link>
                        </li>
                        <li>
                            <Link href="https://twitter.com/HaneulNetwork">
                                Twitter
                            </Link>
                        </li>
                        <li>
                            <Link href="https://www.linkedin.com/company/haneullabs-labs/">
                                LinkedIn
                            </Link>
                        </li>
                    </ul>
                </div>
                <div>
                    <h6>Legal</h6>
                    <ul>
                        <li>
                            <Link href="https://haneul-labs.com/legal?content=terms">
                                Terms & Conditions
                            </Link>
                        </li>
                        <li>
                            <Link href="https://haneul-labs.com/legal?content=privacy">
                                Privacy Policy
                            </Link>
                        </li>
                    </ul>
                </div>
            </nav>
            <div className={styles.logomobile}>
                <HaneulLogoIcon />
                <div className={styles.copyright}>
                    <div>&copy;2022 Haneul. All rights reserved.</div>
                </div>
            </div>
        </footer>
    );
}

export default Footer;
