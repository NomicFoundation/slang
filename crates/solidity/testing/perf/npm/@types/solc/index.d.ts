declare module "solc" {
  export interface Solc {
    compile(input: string, options: any): string;
  }

  export interface SolcLoader {
    loadRemoteVersion(version: string, callback: (err: Error | null, solc: Solc) => void): void;
  }

  const solc: SolcLoader;
  export default solc;
}
