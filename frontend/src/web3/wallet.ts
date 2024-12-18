// import {
//   cipher,
//   decryptWithPrivateKey,
//   encryptWithPublicKey,
// } from "eth-crypto";
import { Address, getAddress, hexToNumber } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { CHAINID } from "./constants";

export type SignResult = {
  message: string;
  signature: `0x${string}`;
  [key: string]: any;
};

class EvmWallet {
  private provider: any;

  public chainId = 0;
  public address = "";

  private init(): void {
    if ("ethereum" in window) {
      if (!this.provider) {
        const provider = window.ethereum;
        this.provider = provider;
      }
    } else {
      window.open("https://metamask.io/", `_blank`);
    }
  }

  getPublicKey(privateKey: `0x${string}`) {
    const account = privateKeyToAccount(privateKey);
    return account.publicKey;
  }
  getAccount(privateKey: `0x${string}`): Address {
    const account = privateKeyToAccount(privateKey);
    return account.address;
  }
  getAddress(address: string) {
    return getAddress(address);
  }

  async connect(): Promise<boolean> {
    if (!this.provider) {
      return false;
    }

    const accounts = await this.provider.request({
      method: "eth_requestAccounts",
    });
    const chainIdHex = await this.provider.request({
      method: "eth_chainId",
      params: [],
    });

    this.chainId = hexToNumber(chainIdHex);
    this.address = getAddress(accounts[0]);

    return true;
  }

  private async getNonce() {
    const nonce = await this.provider.request({
      method: "eth_getTransactionCount",
      params: [this.address, "latest"],
    });
    return parseInt(nonce, 16).toString();
  }
  private async getBlockNumber() {
    const blockNumber = await this.provider.request({
      method: "eth_blockNumber",
      params: [],
    });
    return parseInt(blockNumber, 16).toString();
  }

  // private getNonce2() {
  //   return customAlphabet("abcdefghijklmnopqrstuvwxyz0123456789")(12);
  // }

  async initAndConnect() {
    this.init();
    await this.connect();
  }

  async signByEIP4361(statement: string): Promise<SignResult> {
    await this.initAndConnect();
    const nonce = await this.getBlockNumber();
    const chainId = this.chainId;
    console.log({
      a: chainId !== Number(CHAINID),
      chainId,
      CHAINID: Number(CHAINID),
    });
    if (chainId !== Number(CHAINID)) {
      throw Error("Please try late~");
    }
    const loginMsg = {
      domain: location.host, // "0.0.0.0:9098", //
      chainId: this.chainId, // 31337, // this.chainId,
      origin: location.origin, // "http://0.0.0.0:9098/login", //
      issuedAt: new Date().toISOString(),
      version: "1",
      nonce: nonce,
      statement,
      signType: "EIP-4361",
      address: evmWallet.address,
    };
    return this.sign(JSON.stringify(loginMsg));
  }

  async sign(message: string): Promise<SignResult> {
    const msgData = JSON.parse(message);
    console.log({ msgData });
    const fullMessage =
      msgData.signType === "EIP-4361"
        ? this.to4361Message({
            domain: msgData.domain,
            chainId: msgData.chainId || "",
            uri: msgData.origin,
            issuedAt: msgData.issuedAt,
            version: msgData.version,
            nonce: msgData.nonce,
            statement: msgData.statement,
            address: msgData.address,
          })
        : message;
    const sign = await this.provider.request({
      method: "personal_sign",
      params: [fullMessage, this.address],
    });
    return {
      message: fullMessage,
      signature: sign as `0x${string}`,
      ...msgData,
    };
  }

  // EIP-4361 formated message, ready for EIP-191 signing.
  private to4361Message({
    domain,
    version,
    nonce,
    issuedAt,
    statement,
    chainId,
    uri,
    address,
  }: {
    domain: string;
    version: string;
    nonce: string;
    issuedAt: string;
    statement: string;
    chainId: number;
    uri: string;
    address: string;
  }): string {
    const header = `${domain} wants you to sign in with your Ethereum account:`;
    const uriField = `URI: ${uri}`;
    let prefix = [header, address].join("\n");
    const versionField = `Version: ${version}`;

    // const addressField = `Address: ` + address;

    const chainField = `Chain ID: ` + chainId || "1";

    const nonceLength = nonce.length;
    let _nonce = nonce;
    if (nonceLength < 8) {
      _nonce = "0".repeat(8 - nonceLength) + nonce;
    }

    const nonceField = `Nonce: ${_nonce}`;

    const suffixArray = [uriField, versionField, chainField, nonceField];

    suffixArray.push(`Issued At: ${issuedAt}`);

    const suffix = suffixArray.join("\n");
    prefix = [prefix, statement].join("\n\n");
    if (statement) {
      prefix += "\n";
    }
    return [prefix, suffix].join("\n");
  }

  async signByEIP712(message: string): Promise<SignResult> {
    await this.initAndConnect();
    const sign = await this.provider.request({
      method: "eth_signTypedData_v4",
      params: [this.address, message],
    });
    return {
      message,
      signature: sign,
    };
  }

  // async encryptWithPublicKey(text: string, pubKey: string) {
  //   const finalPubKey = pubKey.startsWith("0x") ? pubKey.slice(2) : pubKey;
  //   const encrypted = await encryptWithPublicKey(finalPubKey, text);
  //   const result = cipher.stringify(encrypted);
  //   return result;
  // }

  // async decryptWithPrivateKey(text: string, privateKey: string) {
  //   const encrypted = cipher.parse(text);
  //   return await decryptWithPrivateKey(privateKey, encrypted);
  // }

  async getIdentityByWallet(seed: string) {
    const signResult = await evmWallet.signByEIP712(
      JSON.stringify({
        types: {
          EIP712Domain: [{ name: "name", type: "string" }],
          Info: [{ name: "seed", type: "string" }],
        },
        primaryType: "Info",
        domain: {
          name: "Secure Note",
        },
        message: {
          seed,
        },
      })
    );
    const privateKey = signResult.signature.slice(0, 66) as `0x${string}`;
    const publicKey = evmWallet.getPublicKey(privateKey);

    return {
      privateKey,
      publicKey,
    };
  }
}

export const evmWallet = new EvmWallet();
