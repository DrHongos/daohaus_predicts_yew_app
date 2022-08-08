export function getProvider() {
  const provider = new ethers.providers.Web3Provider(window.ethereum)
  
  // MetaMask requires requesting permission to connect users accounts
  //await provider.send("eth_requestAccounts", []);
  
  // The MetaMask plugin also allows signing transactions to
  // send ether and pay to change state within the blockchain.
  // For this, you need the account signer...
  //const signer = provider.getSigner()
  // what to return? And how to store it in rust?
  //console.log(`provider ${provider}`);

  // it answers fake data.. pretty interesting
  Object.entries(provider).forEach(keyValuePair => {console.log("  ",...keyValuePair)})

  return "provider";
}
  
export function callEthers(callback) {
  callback(getProvider());
}

