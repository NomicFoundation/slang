declare module "solc07" {
  class Compiler {
    compile(input: string, options: Object): string;
  }

  export default Compiler;
}
