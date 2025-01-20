import {
  createPublicClient,
  createWalletClient,
  custom,
  parseEther,
  parseGwei,
  Address,
  Abi,
  TypedDataDomain,
  parseSignature,
  encodeFunctionData,
  PublicClient,
  http,
  Transport,
} from "viem";
import { chain } from "@/web3/wagmi.config";

class ContractService {
  private publicClient: PublicClient<Transport, typeof chain>;
  private contractAddress?: `0x${string}`;
  private contractABI?: any[];
  private walletClient: ReturnType<typeof createWalletClient> | null = null;
  constructor(contractAddress?: `0x${string}`, contractABI?: any[]) {
    this.publicClient = createPublicClient({ chain, transport: http() });
    this.contractAddress = contractAddress;
    this.contractABI = contractABI;
  }
  private async getWalletClient(): Promise<
    ReturnType<typeof createWalletClient>
  > {
    if (!this.walletClient) {
      this.walletClient = createWalletClient({
        chain,
        transport: custom(window?.ethereum!),
      });
    }
    return this.walletClient;
  }
  private async getWalletAddress(): Promise<Address> {
    const walletClient = await this.getWalletClient();
    const [account] = await walletClient.getAddresses();
    return account;
  }
  async writeContractMethod(
    methodName: string,
    args: any[],
    overrides?: { value?: string; gasLimit?: string; gasPrice?: string }
  ) {
    try {
      const walletClient = await this.getWalletClient();
      const account = await this.getWalletAddress();

      const data = encodeFunctionData({
        abi: this.contractABI ?? [],
        functionName: methodName,
        args,
      });

      const gas = await this.publicClient?.estimateGas({
        account,
        to: this.contractAddress!,
        data,
        value: overrides?.value ? parseEther(overrides.value) : 0n,
      })
    
      const { request } = await this.publicClient?.simulateContract({
        account: account,
        address: this.contractAddress!,
        abi: this.contractABI!,
        functionName: methodName,
        args,
        chain: walletClient.chain,
        ...(overrides?.value !== undefined && {
          value: parseEther(overrides.value),
        }),
        ...(overrides?.gasLimit !== undefined ? { gas: parseGwei(overrides.gasLimit) } : { gas }),
        ...(overrides?.gasPrice !== undefined && {
          gasPrice: parseGwei(overrides.gasPrice),
        }),
      });

      const hash = await walletClient.writeContract(request);
      const transaction = await this.publicClient?.waitForTransactionReceipt({
        hash,
        confirmations: 2
      });
      return transaction.transactionHash;
    } catch (err: any) {
      console.log({ err });
      throw err;
    }
  }
  async signTypedData(): Promise<{
    v: number;
    r: `0x${string}`;
    s: `0x${string}`;
  }> {
    try {
      const walletClient = await this.getWalletClient();
      const account = await this.getWalletAddress();
      const domain: TypedDataDomain = {
        name: "My DApp",
        version: "1.0",
        chainId: chain.id,
        verifyingContract: this.contractAddress!,
      };

      const types = {
        Message: [
          { name: "text", type: "string" },
          { name: "number", type: "uint256" },
        ],
      };
      const signature = await walletClient.signTypedData({
        domain,
        types,
        account,
        primaryType: "Message",
        message: {
          text: "Hello, Ethereum!",
          number: 42,
        },
      });
      const { v, r, s } = parseSignature(signature);
      return { v: Number(v), r, s };
    } catch (err) {
      console.error("Error in signTypedData:", err);
      throw err;
    }
  }
  async readContractData(methodName: string, args: any[]) {
    if (!this.contractAddress || !this.contractABI) {
      throw new Error("Contract address and ABI must be provided");
    }
    const result = await this.publicClient?.readContract({
      address: this.contractAddress,
      abi: this.contractABI,
      functionName: methodName,
      args,
    });
    return result;
  }
  async readContractDataBatch(
    params: {
      address: Address;
      abi: any;
      functionName: string;
      args?: any[];
    }[]
  ): Promise<any[]> {
    try {
      return this.publicClient?.multicall({
        contracts: params.map((param) => {
          return {
            address: param.address,
            abi: param.abi as Abi,
            functionName: param.functionName,
            args: param.args,
          };
        }),
      });
    } catch (error) {
      console.error("Error in readContractDataBatch:", error);
      throw error;
    }
  }
}

export default ContractService;
