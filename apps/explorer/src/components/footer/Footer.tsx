// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ReactComponent as HaneulLogoIcon } from '../../assets/Haneul Logo.svg';
import { CookiesConsent } from '../cookies-consent/CookiesConsent';
import ExternalLink from '../external-link/ExternalLink';

import styles from './Footer.module.css';

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
                            <ExternalLink
                                href="https://medium.com/haneullabs-labs"
                                label="Blog"
                            />
                        </li>
                        <li>
                            <ExternalLink
                                href="https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf"
                                label="Whitepaper"
                            />
                        </li>
                    </ul>
                </div>
                <div>
                    <h6>Build</h6>
                    <ul>
                        <li>
                            <ExternalLink
                                href="https://docs.haneul.io/"
                                label="Docs"
                            />
                        </li>
                        <li>
                            <ExternalLink
                                href="https://github.com/GeunhwaJeong"
                                label="GitHub"
                            />
                        </li>
                        <li>
                            <ExternalLink
                                href="https://discord.gg/haneul"
                                label="Discord"
                            />
                        </li>
                    </ul>
                </div>
                <div>
                    <h6>Follow</h6>
                    <ul>
                        <li>
                            <ExternalLink
                                href="https://haneul-labs.com/#community"
                                label="Press"
                            />
                        </li>
                        <li>
                            <ExternalLink
                                href="https://twitter.com/haneul_labs"
                                label="Twitter"
                            />
                        </li>
                        <li>
                            <ExternalLink
                                href="https://www.linkedin.com/company/haneullabs-labs/"
                                label="LinkedIn"
                            />
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

            <CookiesConsent />
        </footer>
    );
}

export default Footer;
