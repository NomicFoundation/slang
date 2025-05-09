declare module "solc08" {
  class Compiler {
    compile(input: string, options: Object): string;
  }

  export default Compiler;
}
