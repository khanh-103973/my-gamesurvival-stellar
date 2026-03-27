import {
  SorobanRpc,
  TransactionBuilder,
  Networks,
  Operation,
  nativeToScVal,
} from "@stellar/stellar-sdk";

import { signTransaction } from "@stellar/freighter-api";

const server = new SorobanRpc.Server(
  "https://soroban-testnet.stellar.org"
);

const CONTRACT_ID = "YOUR_CONTRACT_ID";

export async function callChoose(publicKey, skill, stress, exp) {
  // 1. Lấy account
  const account = await server.getAccount(publicKey);

  // 2. Build transaction
  const tx = new TransactionBuilder(account, {
    fee: "100",
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(
      Operation.invokeHostFunction({
        function: {
          type: "invokeContract",
          contract: CONTRACT_ID,
          function: "choose",
          args: [
            nativeToScVal(skill),
            nativeToScVal(stress),
            nativeToScVal(exp),
          ],
        },
      })
    )
    .setTimeout(30)
    .build();

  // 3. Ký bằng Freighter
  const signed = await signTransaction(tx.toXDR(), {
    networkPassphrase: Networks.TESTNET,
  });

  // 4. Gửi transaction
  const txSigned = TransactionBuilder.fromXDR(
    signed,
    Networks.TESTNET
  );

  const result = await server.sendTransaction(txSigned);

  return result;
}