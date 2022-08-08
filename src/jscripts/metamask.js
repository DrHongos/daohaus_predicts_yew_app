export async function callEthersAsync() {
    // should return an object that i can parse into a ethers(rs)::Signer
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    await provider.send("eth_requestAccounts", []);
    const signer = provider.getSigner();
    const signerAddress = await signer.getAddress();
    return signerAddress;
}