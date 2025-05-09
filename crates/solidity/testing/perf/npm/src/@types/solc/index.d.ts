declare module "solc" {
  export const solc: {
    loadRemoteVersion: (version: string,
      callback: (err: Error,
        snapshot: { compile: (input: string, options: any) => string, }
      ) => void) => void;
  }


  export default solc;
}
