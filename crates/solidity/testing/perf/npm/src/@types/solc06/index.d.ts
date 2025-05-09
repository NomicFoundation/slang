declare module "solc06" {
  class Compiler {
    compile(input: string, options: Object): string;
  }

  export default Compiler;
}
