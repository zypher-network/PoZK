import api from "@/lib/fetch";
import { stringify } from 'querystring';
import { Address } from "viem";

type ProveContainer = {
  created: number;
  image: string;
  name: string;
  prover: string;
  tag: string;
}

class PoZK {
  endpoints = {
    login: '/login',
    proverImage: '/api/provers/{id}',
    controller: {
      new: '/api/controllers',
      list: '/api/controllers',
    },
    prover: {
      list: '/api/provers',
      pull: '/api/provers',
    },
  }

  async getProverContainers (page: number = 1, pageSize: number = 10): Promise<ProveContainer[]> {
    try {
      const params = stringify({ page_count: page, page_size: pageSize });
      const data = await api.get(`${this.endpoints.prover.list}?${params}`) as any;
      return data?.data ?? [];
    } catch (error) {
      return this.handleError(error as Error, []);
    }
  }

  // async getProverImage (prover: string, page: number = 1, pageSize: number = 10): Promise<ProverImageResponse | null> {
  //   try {
  //     const params = stringify({ page_count: page, page_size: pageSize });
  //     return await api.get(`${this.endpoints.proverImage.replace('{id}', prover)}?${params}`) as ProverImageResponse;
  //   } catch (error) {
  //     return this.handleError(error as Error, null);
  //   }
  // }

  async newController (): Promise<Address | ''> {
    try {
      const data = await api.post(this.endpoints.controller.new, {"singing_key": null}) as any;
      console.log(data);
      return data?.controller ?? '';
    } catch (error) {
      return this.handleError(error as Error, '');
    }
  }

  async getControllers (page: number = 1, pageSize: number = 10): Promise<Address[]> {
    const params = stringify({ page_count: page, page_size: pageSize });
    try {
      const data = await api.get(`${this.endpoints.controller.list}?${params}`) as any;
      return data?.data ?? [];
    } catch (error) {
      return this.handleError(error as Error, []);
    }
  }

  async getActiveController (): Promise<Address | ''> {
    const params = stringify({ page_count: 0, page_size: 0 });
    try {
      const data = await api.get(`${this.endpoints.controller.list}?${params}`) as any;
      return data?.main ?? '';
    } catch (error) {
      return this.handleError(error as Error, '');
    }
  }

  async login (params: any) {
    return await api.post(`${this.endpoints.login}`, params);
  }

  async pullProve (prover: Address, tag: string, name: string, overtime: string) {
    try {
      const req = {
        tag: `v${tag}`,
        name,
        prover,
        overtime: Number(overtime),
      };
      const response = await api.post(`${this.endpoints.prover.pull}`, req) as any;
    } catch (error) {
      this.handleError(error as Error, '');
    }
  }

  // async newProve (prover: Address, tag: string, name: string) {
  //   try {
  //     const req = {
  //       option: {
  //         env: [
  //           `INPUT=/data/${prover}.input`,
  //           `OUTPUT=/data/${prover}.publics`,
  //           `PROOF=/data/${prover}.proof`
  //         ],
  //         volumes: [
  //           {
  //             "src_volumes": "/data",
  //             // "host_volumes": '/home/ubuntu/tmp'
  //             "host_volumes": `/home/cloud/zypher/pozk-${name}/prover/examples`
  //           }
  //         ]
  //       },
  //       prover,
  //       tag: `v${tag}`,
  //     };
  //     const response = await api.post(`${this.endpoints.prover.new}`, req) as any;
  //     console.log(response)
  //   } catch (error) {
  //     this.handleError(error as Error, '');
  //   }
  // }

  private handleError<T> (error: Error, defaultReturn: T): T {
    const { message } = error;
    console.error('catch error:', message, message.includes('500'));
    if (message.includes('500')) {
      return defaultReturn;
    }
    throw error;
  }
}

export default new PoZK();
