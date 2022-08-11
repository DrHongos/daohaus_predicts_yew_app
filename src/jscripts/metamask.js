
export async function connectMetamask() {
    // should return an object that i can parse into a ethers(rs)::Signer
    const provider = new ethers.providers.Web3Provider(window.ethereum);
//    if(provider) Object.entries(provider).forEach(keyValuePair => {console.log("  ",...keyValuePair)})
    await provider.send("eth_requestAccounts", []);
    const signer = provider.getSigner();
    const signerAddress = await signer.getAddress();

    // i need to type correctly this to send a JsonRpcProvider type

    //const chainId = JSON.stringify(signer); // circulare reference
    //console.log(`signer chainId ${signer}`)
    //Object.entries(signer.provider).forEach(keyValuePair => {console.log("  ",...keyValuePair)})
    return signerAddress;
}
export async function getProvider() {
    // should return an object that i can parse into a ethers(rs)::JsonRpcProvider
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    await provider.send("eth_requestAccounts", []);
    let providerWeb3 = provider._web3Provider;
    //if(provider) Object.entries(providerWeb3).forEach(keyValuePair => {console.log("  ", ...keyValuePair)})
    //let signer = await provider.getSigner();
    // maybe create a class workable? otherwise its difficult to 
    // deserialize in rust side... 

    return providerWeb3;
}
export async function signMessage() {
    const provider = await getProvider();
    const signer = await provider.getSigner();
    const flatSignature = await signer.signMessage("Hello js in browser");
    console.log(`signed: ${flatSignature}`);
    return flatSignature;
}