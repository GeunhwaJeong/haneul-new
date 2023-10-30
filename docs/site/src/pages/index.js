// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import React, { useState } from "react";
import { useHistory } from "@docusaurus/router";

import Layout from "@theme/Layout";
import Link from "@docusaurus/Link";

export default function Home() {
  const history = useHistory();
  const SingleLink = (props) => {
    const { to } = props;

    return (
      <div
        onClick={() => history.push(to)}
        className="bg-haneul-card-dark rounded-haneul w-[350px] h-[350px] p-8 bg-[url(../static/img/index/card-bg.svg)] justify-center flex justify-center items-center"
      >
        <div className="p-4 rounded-full border border-solid border-haneul-white/30 w-[249px] h-[51px] text-haneul-white bg-haneul-card-dark bg-[url(../static/img/index/circle-arrow.svg)] bg-no-repeat bg-[center_right_2px] flex items-center cursor-pointer hover:shadow-haneul hover:shadow-haneul-blue hover:bg-opacity-50 hover:bg-[url(../static/img/index/circle-arrow-blue.svg)]">
          Build your first dApp
        </div>
      </div>
    );
  };
  const ContentItem = (props) => {
    const [vis, setVis] = useState(false);
    const { children, title } = props;
    const handleClick = () => {
      setVis(!vis);
    };

    return (
      <div className="grid grid-cols-3 border-solid border-0 border-t border-haneul-white/50 mb-8 lg:mx-0 mx-4">
        <p
          className={`lg:text-4xl text-2xl pb-2 mt-8 pr-12 cursor-pointer bg-no-repeat bg-right-top flex-none ${
            vis
              ? "bg-[url(../static/img/index/circle-arrow-up.svg)]"
              : "bg-[url(../static/img/index/circle-arrow-down.svg)]"
          }`}
          onClick={handleClick}
        >
          {title}
        </p>
        <p
          className={`${
            vis ? "opacity-100" : "opacity-0"
          } transition-opacity ease-in-out duration-300 col-span-2 mt-10 text-lg text-haneul-blue-lighter mx-8`}
        >
          {vis && children}
        </p>
      </div>
    );
  };
  const HomeCard = (props) => {
    const { aux, title, children } = props;
    return (
      <div
        className={`p-px col-span-3 bg-gradient-to-b from-haneul-white/40 from-20% hover:from-30% via-haneul-white/0 to-haneul-white/10 to-80% rounded-haneul w-[350px] h-[350px]`}
      >
        <div
          className={`${
            aux ? "bg-haneul-ghost-dark" : "bg-haneul-card-dark"
          } rounded-haneul w-full h-full p-8 max-w-[350px] max-h-[350px]`}
        >
          <p className="text-4xl text-white table-cell align-bottom pb-2 h-16 pb-8">
            {title}
          </p>
          {children}
        </div>
      </div>
    );
  };

  const cardlinks =
    "block py-3 border-0 border-t border-solid border-color-white text-haneul-blue-lighter bg-[url(../static/img/index/right-arrow.svg)] bg-no-repeat bg-right hover:no-underline hover:text-haneul-blue-lighter hover:bg-[url(../static/img/index/right-arrow-blue.svg)] pr-10";
  const darkcardclass =
    "p-px col-span-4 bg-gradient-to-b from-haneul-white/40 from-20% hover:from-30% via-haneul-white/0 to-haneul-white/10 to-80% rounded-haneul";
  return (
    <Layout>
      <div className="bg-haneul-black overflow-hidden">
        <div className="w-full mt-24 mb-12 mx-auto bg-haneul-black">
          <div className="text-center">
            <p className="lg:text-8xl text-6xl text-white">Haneul Documentation</p>
            <p className="xs:text-md sm:text-xl lg:text-3xl mb-0 w-1/3 m-x-auto text-haneul-blue-lighter inline-block">
              Discover the power of Haneul through examples, guides, and concepts
            </p>
          </div>
        </div>
        <div className="flex flex-row flex-wrap justify-center gap-2 max-w-[1066px] mx-auto">
          <HomeCard title="About Haneul">
            <Link
              className={`${cardlinks} text-2xl`}
              to="./concepts/tokenomics"
            >
              Tokenomics
            </Link>
            <Link
              className={`${cardlinks} text-2xl`}
              to="./concepts/cryptography"
            >
              Cryptography
            </Link>
            <Link className={`${cardlinks} text-2xl`} to="standards">
              Standards
            </Link>
          </HomeCard>
          <HomeCard title="Developers">
            <Link
              className={`${cardlinks} text-2xl`}
              to="./guides/developer/getting-started"
            >
              Getting started
            </Link>
            <Link
              className={`${cardlinks} text-2xl`}
              to="./guides/developer/haneul-101"
            >
              Haneul Developer Basics
            </Link>
            <Link
              className={`${cardlinks} text-2xl`}
              to="./concepts/haneul-move-concepts"
            >
              Move
            </Link>
          </HomeCard>
          <HomeCard title="Validators and Node operators">
            <Link
              className={`${cardlinks} text-2xl`}
              to="./guides/operator/validator-config"
            >
              Validator configuration
            </Link>
            <Link
              className={`${cardlinks} text-2xl`}
              to="./guides/operator/haneul-full-node"
            >
              Run a Haneul Full node
              <span className="block bg-auto bg-[url(../static/img/index/right-arrow.svg)]"></span>
            </Link>
          </HomeCard>
          <HomeCard title="References" aux>
            <Link
              className={`${cardlinks} text-md`}
              to="https://haneul-typescript-docs.vercel.app/dapp-kit?ref=blog.haneul.io"
            >
              Haneul dApp Kit
            </Link>
            <Link className={`${cardlinks} text-md`} to="/haneul-api-ref">
              Haneul API
            </Link>
            <Link
              className={`${cardlinks} text-md`}
              to="https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-framework/docs"
            >
              Haneul framework (GitHub)
            </Link>
            <Link
              className={`${cardlinks} text-md`}
              to="https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-sdk"
            >
              Rust SDK (GitHub)
            </Link>
          </HomeCard>
          <HomeCard title="Resources" aux>
            <Link
              className={`${cardlinks} text-md`}
              to="https://haneul.directory/"
            >
              Haneul partner packages
            </Link>
            <Link className={`${cardlinks} text-md`} to="https://blog.haneul.io/">
              Haneul blog
            </Link>
            <Link
              className={`${cardlinks} text-md`}
              to="guides/developer/dev-cheat-sheet"
            >
              Haneul dev cheat sheet
            </Link>
          </HomeCard>
          <div className={`${darkcardclass} w-[350px] h-[350px]`}>
            <SingleLink to="/guides/developer/first-app"></SingleLink>
          </div>
        </div>

        <div className="lg:w-[1066px] w-auto mt-24 text-white mx-auto">
          <h1 className="mb-4 text-8xl font-normal text-center">Why Haneul?</h1>
          <p className="text-3xl mb-8 text-haneul-blue-lighter inline-block text-center w-full">
            <span className="inline-block sm:w-[578px] w-[500px] mb-6">
              Haneul is the first internet-scale programmable blockchain platform
            </span>
          </p>
        </div>
        <div className="lg:w-[1066px] width-auto mx-auto flex flex-row items-center">
          <div className="lg:w-[350px] w-[250px] mx-auto lg:mx-0 ">
            <img src="/img/index/blocks.png"/>
          </div>
          <div className="lg:w-[676px] ml-8 lg:block hidden">
            <p className="text-xl text-haneul-blue-lighter border-solid border-0 border-t border-haneul-white/50">
              <span className="inline-block mt-7 mb-3 ml-4">
                Unmatched scalability, instant settlement
              </span>
            </p>
            <p className="text-xl text-haneul-blue-lighter border-solid border-0 border-t border-haneul-white/50">
              <span className="inline-block mt-7 mb-3 ml-4">
                A safe smart contract language accessible to mainstream
                developers
              </span>
            </p>
            <p className="text-xl text-haneul-blue-lighter border-solid border-0 border-t border-haneul-white/50">
              <span className="inline-block mt-7 mb-3 ml-4">
                Ability to define rich and composable on-chain assets
              </span>
            </p>
            <p className="text-xl text-haneul-blue-lighter border-solid border-0 border-t border-haneul-white/50">
              <span className="inline-block mt-7 mb-3 ml-4">
                Better user experience for web3 apps
              </span>
            </p>
          </div>
        </div>
        <div className="sm:w-[840] lg:w-[1066px] w-auto my-24 text-white mx-auto">
          <ContentItem title="Scalability">
            Haneul scales horizontally to meet the demands of applications. Network
            capacity grows in proportion to the increase in Haneul validators'
            processing power by adding workers, resulting in low gas fees even
            during high network traffic. This scalability characteristic is in
            sharp contrast to other blockchains with rigid bottlenecks.
          </ContentItem>
          <ContentItem title="Move">
            Move design prevents issues such as reentrancy vulnerabilities,
            poison tokens, and spoofed token approvals that attackers have
            leveraged to steal millions on other platforms. The emphasis on
            safety and expressivity provides a more straightforward transition
            from web 2.0 to web3 for developers, without the need to understand
            the intricacies of the underlying infrastructure.
          </ContentItem>
          <ContentItem title="On-chain assets">
            Rich on-chain assets enable new applications and economies based on
            utility without relying solely on artificial scarcity. Developers
            can implement dynamic NFTs that you can upgrade, bundle, and group
            in an application-specific manner, such as changes in avatars and
            customizable items based on gameplay. This capability delivers
            stronger in-game economies as NFT behavior gets fully reflected
            on-chain, making NFTs more valuable and delivering more engaging
            feedback loops.
          </ContentItem>
          <ContentItem title="Built for Web3">
            Haneul aims to be the most accessible smart contract platform,
            empowering developers to create great user experiences in web3. To
            usher in the next billion users, Haneul empowers developers with
            various tools to take advantage of the power of the Haneul blockchain.
            The Haneul Development Kit (SDK) will enable developers to build
            without boundaries.
          </ContentItem>
        </div>
      </div>
    </Layout>
  );
}
