const VIRTUAL_FS = new Map<string, string>([
  [
    "contract.sol",
    `
      import { Log } from "events.sol";

      contract MyContract {
        function test() public {
          emit Log(msg.sender, "Hello World!");
        }
      }
    `,
  ],
  [
    "events.sol",
    `
      event Log(address indexed sender, string message);
    `,
  ],
]);

export async function readFile(fileId: string) {
  return VIRTUAL_FS.get(fileId);
}
