import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { WeightV2 } from "@polkadot/types/interfaces";

const WS_PROVIDER = "wss://ws.test.azero.dev"; // Aleph Zero Testnet
const ESCROW_CONTRACT_ADDRESS =
  "5FHZ8i317KBNRe8CtKi478TKS1YGZKQcH8YDTz8StqEHLW7N"; // Adres kontraktu Escrow
const PSP22_CONTRACT_ADDRESS =
  "5CqrEEZtZ7ShQZ5RabcatUu2s634efUGTg3Qj7822gAe7754"; // Adres kontraktu PSP22
const ESCROW_ABI = require("./escrow.json");
const PSP22_ABI = require("./psp22.json");

async function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
  await cryptoWaitReady();
  console.log("Crypto ready");

  // Connect to the node
  const provider = new WsProvider(WS_PROVIDER);
  const api = await ApiPromise.create({ provider });
  console.log("Connected to the node");

  // Initialize keyring
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");
  console.log("Keyring initialized");

  // Initialize contracts
  const escrowContract = new ContractPromise(
    api,
    ESCROW_ABI,
    ESCROW_CONTRACT_ADDRESS
  );
  const psp22Contract = new ContractPromise(
    api,
    PSP22_ABI,
    PSP22_CONTRACT_ADDRESS
  );
  console.log("Contracts initialized");

  // Display available methods
  console.log(
    "Available methods in PSP22 contract:",
    Object.keys(psp22Contract.tx)
  );

  // Define gas limit as a number
  const gasLimit = 2000000000; // Bezpośrednia wartość liczbową

  // Approve tokens for escrow contract
  const approveTx = psp22Contract.tx["psp22::approve"](
    { value: 0, gasLimit },
    ESCROW_CONTRACT_ADDRESS,
    1000
  );
  const approveResult = await approveTx.signAndSend(alice);
  console.log("Approve Result:", approveResult);

  // Delay before next transaction
  await delay(10000); // 10 sekund

  // Deposit tokens into escrow
  const depositTx = escrowContract.tx.deposit({ value: 0, gasLimit }, 1000);
  const depositResult = await depositTx.signAndSend(alice);
  console.log("Deposit Result:", depositResult);

  // Delay before next transaction
  await delay(10000); // 10 sekund

  // Check balance in escrow
  const balanceResult = await escrowContract.query.getBalance(
    alice.address,
    { gasLimit },
    alice.address
  );
  console.log("Balance Query Result:", balanceResult);

  if (balanceResult.output !== null && balanceResult.result.isOk) {
    const escrowBalance = balanceResult.output.toHuman();
    console.log("Escrow Balance:", escrowBalance);
  } else {
    console.error(
      "Error querying balance:",
      balanceResult.result.asErr?.toString()
    );
  }

  // Delay before next transaction
  await delay(10000); // 10 sekund

  // Withdraw tokens from escrow
  const withdrawTx = escrowContract.tx.withdraw({ value: 0, gasLimit }, 500);
  const withdrawResult = await withdrawTx.signAndSend(alice);
  console.log("Withdraw Result:", withdrawResult);

  // Delay before next transaction
  await delay(10000); // 10 sekund

  // Check final balance in escrow
  const finalBalanceResult = await escrowContract.query.getBalance(
    alice.address,
    { gasLimit },
    alice.address
  );
  console.log("Final Balance Query Result:", finalBalanceResult);

  if (finalBalanceResult.output !== null && finalBalanceResult.result.isOk) {
    const finalEscrowBalance = finalBalanceResult.output.toHuman();
    console.log("Final Escrow Balance:", finalEscrowBalance);
  } else {
    console.error(
      "Error querying final balance:",
      finalBalanceResult.result.asErr?.toString()
    );
  }
}

main()
  .catch(console.error)
  .finally(() => process.exit());
