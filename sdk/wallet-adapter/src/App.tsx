// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import "./App.css";
import { useMemo } from "react";
import { Wallet, WalletProvider } from "@haneullabs/wallet-adapter-react";
import { HaneulWalletAdapter } from "@haneullabs/wallet-adapter-all-wallets";
import { WalletStandardAdapterProvider } from "@haneullabs/wallet-adapter-wallet-standard";
import { WalletWrapper } from "@haneullabs/wallet-adapter-react-ui";
import { TestButton } from "./TestButton";

function App() {
  const adapters = useMemo(
    () => [new HaneulWalletAdapter(), new WalletStandardAdapterProvider()],
    []
  );

  return (
    <div className="App">
      <header className="App-header">
        <WalletProvider adapters={adapters}>
          <TestButton />
          <WalletWrapper />
        </WalletProvider>
      </header>
    </div>
  );
}

export default App;
