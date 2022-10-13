// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Base64DataBuffer } from '../../serialization/base64';
import {
  bcs,
  Coin,
  PAY_JOIN_COIN_FUNC_NAME,
  PAY_MODULE_NAME,
  HANEUL_PACKAGE_ID,
  PAY_SPLIT_COIN_VEC_FUNC_NAME,
  HaneulAddress,
  Transaction,
  TransactionData,
  TypeTag,
} from '../../types';
import {
  MoveCallTransaction,
  MergeCoinTransaction,
  SplitCoinTransaction,
  TransferObjectTransaction,
  TransferHaneulTransaction,
  PublishTransaction,
  TxnDataSerializer,
  PayTransaction,
} from './txn-data-serializer';
import { Provider } from '../../providers/provider';
import { CallArgSerializer } from './call-arg-serializer';

const TYPE_TAG = Array.from('TransactionData::').map((e) => e.charCodeAt(0));

export class LocalTxnDataSerializer implements TxnDataSerializer {
  /**
   * Need a provider to fetch the latest object reference. Ideally the provider
   * should cache the object reference locally
   */
  constructor(private provider: Provider) {}

  async newTransferObject(
    signerAddress: HaneulAddress,
    t: TransferObjectTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const objectRef = await this.provider.getObjectRef(t.objectId);
      const tx = {
        TransferObject: {
          recipient: t.recipient,
          object_ref: objectRef!,
        },
      };
      return await this.constructTransactionData(
        tx,
        // TODO: make `gasPayment` a required field in `TransferObjectTransaction`
        t.gasPayment!,
        t.gasBudget,
        signerAddress
      );
    } catch (err) {
      throw new Error(
        `Error constructing a TransferObject transaction: ${err} args ${JSON.stringify(
          t
        )}`
      );
    }
  }

  async newTransferHaneul(
    signerAddress: HaneulAddress,
    t: TransferHaneulTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const tx = {
        TransferHaneul: {
          recipient: t.recipient,
          amount: t.amount == null ? { None: null } : { Some: t.amount },
        },
      };
      return await this.constructTransactionData(
        tx,
        t.haneulObjectId,
        t.gasBudget,
        signerAddress
      );
    } catch (err) {
      throw new Error(
        `Error constructing a TransferHaneul transaction: ${err} args ${JSON.stringify(
          t
        )}`
      );
    }
  }

  async newPay(
    signerAddress: HaneulAddress,
    t: PayTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const inputCoinRefs = (
        await Promise.all(
          t.inputCoins.map((coin) => this.provider.getObjectRef(coin))
        )
      ).map((ref) => ref!);
      const tx = {
        Pay: {
          coins: inputCoinRefs,
          recipients: t.recipients,
          amounts: t.amounts,
        },
      };
      return await this.constructTransactionData(
        tx,
        t.gasPayment!,
        t.gasBudget,
        signerAddress
      );
    } catch (err) {
      throw new Error(
        `Error constructing a Pay transaction: ${err} args ${JSON.stringify(t)}`
      );
    }
  }

  async newMoveCall(
    signerAddress: HaneulAddress,
    t: MoveCallTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const pkg = await this.provider.getObjectRef(t.packageObjectId);
      const tx = {
        Call: {
          package: pkg!,
          module: t.module,
          function: t.function,
          typeArguments: t.typeArguments as TypeTag[],
          arguments: await new CallArgSerializer(
            this.provider
          ).serializeMoveCallArguments(t),
        },
      };

      return await this.constructTransactionData(
        tx,
        // TODO: make `gasPayment` a required field in `MoveCallTransaction`
        t.gasPayment!,
        t.gasBudget,
        signerAddress
      );
    } catch (err) {
      throw new Error(
        `Error constructing a move call: ${err} args ${JSON.stringify(t)}`
      );
    }
  }

  async newMergeCoin(
    signerAddress: HaneulAddress,
    t: MergeCoinTransaction
  ): Promise<Base64DataBuffer> {
    try {
      return await this.newMoveCall(signerAddress, {
        packageObjectId: HANEUL_PACKAGE_ID,
        module: PAY_MODULE_NAME,
        function: PAY_JOIN_COIN_FUNC_NAME,
        typeArguments: [await this.getCoinStructTag(t.coinToMerge)],
        arguments: [t.primaryCoin, t.coinToMerge],
        gasPayment: t.gasPayment,
        gasBudget: t.gasBudget,
      });
    } catch (err) {
      throw new Error(
        `Error constructing a MergeCoin Transaction: ${err} args ${JSON.stringify(
          t
        )}`
      );
    }
  }

  async newSplitCoin(
    signerAddress: HaneulAddress,
    t: SplitCoinTransaction
  ): Promise<Base64DataBuffer> {
    try {
      return await this.newMoveCall(signerAddress, {
        packageObjectId: HANEUL_PACKAGE_ID,
        module: PAY_MODULE_NAME,
        function: PAY_SPLIT_COIN_VEC_FUNC_NAME,
        typeArguments: [await this.getCoinStructTag(t.coinObjectId)],
        arguments: [t.coinObjectId, t.splitAmounts],
        gasPayment: t.gasPayment,
        gasBudget: t.gasBudget,
      });
    } catch (err) {
      throw new Error(
        `Error constructing a SplitCoin Transaction: ${err} args ${JSON.stringify(
          t
        )}`
      );
    }
  }

  async newPublish(
    signerAddress: HaneulAddress,
    t: PublishTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const tx = {
        Publish: {
          modules: t.compiledModules as ArrayLike<ArrayLike<number>>,
        },
      };
      return await this.constructTransactionData(
        tx,
        // TODO: make `gasPayment` a required field in `PublishTransaction`
        t.gasPayment!,
        t.gasBudget,
        signerAddress
      );
    } catch (err) {
      throw new Error(
        `Error constructing a newPublish transaction: ${err} with args ${JSON.stringify(
          t
        )}`
      );
    }
  }

  private async getCoinStructTag(coinId: string): Promise<TypeTag> {
    const coin = await this.provider.getObject(coinId);
    const coinTypeArg = Coin.getCoinTypeArg(coin);
    if (coinTypeArg == null) {
      throw new Error(`Object ${coinId} is not a valid coin type`);
    }
    return { struct: Coin.getCoinStructTag(coinTypeArg) };
  }

  private async constructTransactionData(
    tx: Transaction,
    gasObjectId: string,
    gasBudget: number,
    signerAddress: HaneulAddress
  ): Promise<Base64DataBuffer> {
    // TODO: mark gasPayment as required in `MoveCallTransaction`
    const gasPayment = await this.provider.getObjectRef(gasObjectId);
    const txData = {
      kind: {
        // TODO: support batch txns
        Single: tx,
      },
      gasPayment: gasPayment!,
      // Need to keep in sync with
      // https://github.com/GeunhwaJeong/haneul/blob/f32877f2e40d35a008710c232e49b57aab886462/crates/haneul-types/src/messages.rs#L338
      gasPrice: 1,
      gasBudget: gasBudget,
      sender: signerAddress,
    };

    return this.serializeTransactionData(txData);
  }

  private serializeTransactionData(
    tx: TransactionData,
    // TODO: derive the buffer size automatically
    size: number = 2048
  ): Base64DataBuffer {
    const dataBytes = bcs.ser('TransactionData', tx, size).toBytes();
    const serialized = new Uint8Array(TYPE_TAG.length + dataBytes.length);
    serialized.set(TYPE_TAG);
    serialized.set(dataBytes, TYPE_TAG.length);
    return new Base64DataBuffer(serialized);
  }
}
