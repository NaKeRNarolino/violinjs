import * as violin from "./violin_js/mod.ts";

const pack: violin.Pack = new violin.Pack(
  "ViolinJS",
  "violinjs",
  "NaKeR",
  "Tests for ViolinJS APIs",
  "./devbp",
  "./devrp",
  "./violin_rs.png"
);

pack.generate();
