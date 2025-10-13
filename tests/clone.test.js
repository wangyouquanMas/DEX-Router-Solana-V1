const {expect} = require("chai");

const RPC_URL = "http://127.0.01:8899";


const MINT_ADDRESS = [
    "So11111111111111111111111111111111111111112",
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
    "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So",
];

const TOKEN_PROGRAM_ID = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

async function getAccountInfo(address) {
    const res = await fetch(RPC_URL,{
        method:"POST",
        headers:{"Content-Type":"application/json"},
        body: JSON.stringify({
            jsonrpc: "2.0",
            id: 1,
            method:"getAccountInfo",
            params:[address,{encoding:"jsonParsed"}],
        }),
    });
    const json = await res.json();
    if (json.error) throw new Error(JSON.stringify(json.error));
    return json.result.value;
}

describe("cloned mint accounts exist on local validator",
    function(){
        this.timeout(60_6000);

        for (const address of MINT_ADDRESS){
            it(`has parsed mint account for ${address}`,async()=>{
                const info = await getAccountInfo(address);
                expect(info, "account should be present").to.not.equal(null);
                expect(info.owner).to.equal(TOKEN_PROGRAM_ID);
                expect(info.data).to.have.property("parsed");
                expect(info.data.parsed).to.have.property("type");
                expect(info.data.parsed.type).to.equal("mint");
            });
        }
    }
);


