
export async function connectMetamask() {
    // should return an object that i can parse into a ethers(rs)::Signer
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    if(provider) Object.entries(provider).forEach(keyValuePair => {console.log("  ",...keyValuePair)})
    await provider.send("eth_requestAccounts", []);
    const signer = provider.getSigner();
    const signerAddress = await signer.getAddress();
    //const chainId = JSON.stringify(signer); // circulare reference
    //console.log(`signer chainId ${signer}`)
    //Object.entries(signer.provider).forEach(keyValuePair => {console.log("  ",...keyValuePair)})
    return signerAddress;
}
export async function getProvider() {
    // should return an object that i can parse into a ethers(rs)::JsonRpcProvider
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    await provider.send("eth_requestAccounts", []);
    return provider;
}
