// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import "./App.css";
import { Wallet, WalletProvider } from "@haneullabs/wallet-adapter-react";
import { HaneulWalletAdapter } from "@haneullabs/wallet-adapter-all-wallets";
import { WalletWrapper } from "@haneullabs/wallet-adapter-react-ui";
import { TestButton } from "./TestButton";

function App() {
  const supportedWallets: Wallet[] = [
    {
      adapter: new HaneulWalletAdapter(),
    },
  ];

  return (
    <div className="App">
      <header className="App-header">
        <WalletProvider supportedWallets={supportedWallets}>
          <TestButton />
          <WalletWrapper />
        </WalletProvider>
      </header>
    </div>
  );
}

export default App;
