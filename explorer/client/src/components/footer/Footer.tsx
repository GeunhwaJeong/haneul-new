import { Link } from 'react-router-dom';

import ExternalLink from '../external-link/ExternalLink';

import styles from './Footer.module.css';

function Footer() {
    return (
        <footer className={styles.footer}>
            <nav className={styles.links}>
                <Link to="/" aria-label="home button">
                    Home
                </Link>
                <ExternalLink
                    href="https://haneul-labs.com/"
                    label="Haneul Labs"
                />
                <ExternalLink
                    href="https://devportal-30dd0.web.app/"
                    label="Developer Hub"
                />
            </nav>
        </footer>
    );
}

export default Footer;
