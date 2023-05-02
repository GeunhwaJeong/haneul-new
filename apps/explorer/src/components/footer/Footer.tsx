// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    SocialDiscord24,
    SocialLinkedin24,
    SocialTwitter24,
} from '@haneullabs/icons';

import { ReactComponent as HaneulWordmark } from '../../assets/HaneulWordmark.svg';

import { Link } from '~/ui/Link';
import { Text } from '~/ui/Text';

function FooterLinks() {
    return (
        <ul className="flex gap-8">
            <li>
                <Link variant="text" href="https://medium.com/haneullabs-labs">
                    <Text variant="body/medium" color="steel-darker">
                        Blog
                    </Text>
                </Link>
            </li>
            <li>
                <Link
                    variant="text"
                    href="https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf"
                >
                    <Text variant="body/medium" color="steel-darker">
                        Whitepaper
                    </Text>
                </Link>
            </li>
            <li>
                <Link variant="text" href="https://haneul-labs.com/#community">
                    <Text variant="body/medium" color="steel-darker">
                        Press
                    </Text>
                </Link>
            </li>
            <li>
                <Link variant="text" href="https://docs.haneul.io/">
                    <Text variant="body/medium" color="steel-darker">
                        Docs
                    </Text>
                </Link>
            </li>
            <li>
                <Link variant="text" href="https://github.com/GeunhwaJeong">
                    <Text variant="body/medium" color="steel-darker">
                        GitHub
                    </Text>
                </Link>
            </li>
            <li>
                <Link variant="text" href="https://discord.gg/haneul">
                    <SocialDiscord24 />
                </Link>
            </li>
            <li>
                <Link variant="text" href="https://twitter.com/HaneulNetwork">
                    <SocialTwitter24 />
                </Link>
            </li>
            <li>
                <Link
                    variant="text"
                    href="https://www.linkedin.com/company/haneullabs-labs/"
                >
                    <SocialLinkedin24 />
                </Link>
            </li>
        </ul>
    );
}

function Footer() {
    return (
        <footer className="bg-gray-40 px-5 py-10 md:px-10 md:py-14">
            <nav className="flex flex-col gap-7.5">
                <div className="flex w-full flex-col items-center justify-between gap-7.5 md:flex-row">
                    <div className="flex gap-2 text-hero-dark">
                        <HaneulWordmark />
                    </div>
                    <FooterLinks />
                </div>

                <div className="h-[1px] w-full bg-gray-45" />
                <div className="flex w-full items-center justify-between">
                    <div className="h-full space-y-2">
                        <Text
                            color="steel-darker"
                            variant="pSubtitleSmall/medium"
                        >
                            &copy;
                            {`${new Date().getFullYear()} Haneul. All
                                rights reserved.`}
                        </Text>
                    </div>
                    <ul className="flex gap-2">
                        <li>
                            <Link
                                variant="text"
                                href="https://haneul-labs.com/legal?content=terms"
                            >
                                <Text
                                    variant="pSubtitleSmall/medium"
                                    color="steel-darker"
                                >
                                    Terms & Conditions
                                </Text>
                            </Link>
                        </li>
                        <li>
                            <Link
                                variant="text"
                                href="https://haneul-labs.com/legal?content=privacy"
                            >
                                <Text
                                    variant="pSubtitleSmall/medium"
                                    color="steel-darker"
                                >
                                    Privacy Policy
                                </Text>
                            </Link>
                        </li>
                    </ul>
                </div>
            </nav>
        </footer>
    );
}

export default Footer;
