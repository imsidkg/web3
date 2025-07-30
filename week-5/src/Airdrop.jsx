import { useConnection, useWallet } from "@solana/wallet-adapter-react";

export function Airdrop() {
  const wallet = useWallet();
  const { connection } = useConnection();

  async function sendAirdropToUser() {
    console.log('reachec')
    await connection.requestAirdrop(wallet.publicKey, 1000000000);
    alert('airdop succedded')
  }
  return (
    <div>
      <input type=" text" placeholder=" Amount" />
      <button onClick={sendAirdropToUser}>Send</button>
    </div>
  );
}
