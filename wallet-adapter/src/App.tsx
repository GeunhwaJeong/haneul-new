// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import './App.css';
import { root } from '.';
import { Wallet, WalletProvider } from 'haneul-wallet-adapter-react';
import { HaneulWalletAdapter, MockWalletAdapter} from '@haneul-wallet-adapter/all-wallets';
import { WalletWrapper } from 'haneul-wallet-adapter-ui';
import { Button } from '@mui/material';
import { TestButton } from './TestButton';

function App() {
  const supportedWallets: Wallet[] = [
    {
      adapter: new HaneulWalletAdapter()
    },
  ];

  return (
    <div className="App">
      <header className="App-header">
         <WalletProvider supportedWallets={supportedWallets}>
          <TestButton/>
          <WalletWrapper/>
        </WalletProvider>
      </header>
    </div>
  );
}

export default App;