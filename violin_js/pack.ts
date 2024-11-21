import "./utils.ts";
import { encode } from "./utils.ts";

const api = Deno.dlopen("./target/debug/violin_rs_js.dll", {
  create_pack: {
    parameters: [
      "buffer",
      "buffer",
      "buffer",
      "buffer",
      "buffer",
      "buffer",
      "buffer",
    ],
    result: "pointer",
  },
  gen_pack: {
    parameters: ["pointer"],
    result: "void",
  },
  add_item: {
    parameters: ["pointer"],
    result: "void",
  },
});

export class Pack {
  private ref: Deno.PointerValue<unknown>;

  public constructor(
    name: string,
    id: string,
    author: string,
    description: string,
    devBpFolder: string,
    devRpFolder: string,
    iconPath: string
  ) {
    this.ref = api.symbols.create_pack(
      encode(name),
      encode(id),
      encode(author),
      encode(description),
      encode(devBpFolder),
      encode(devRpFolder),
      encode(iconPath)
    );
  }

  public generate() {
    api.symbols.gen_pack(this.ref);
  }
}
