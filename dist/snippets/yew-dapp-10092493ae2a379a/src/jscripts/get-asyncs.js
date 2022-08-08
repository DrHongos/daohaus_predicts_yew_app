export async function callEthersAsync() {//callback
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    provider.send("eth_requestAccounts", []);
    //Object.entries(provider).forEach(keyValuePair => {console.log("  ",...keyValuePair)})
    //console.log("async!");
    
    //callback(provider) and "callback" as an argument
}