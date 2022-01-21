import { env } from "@tensorflow/tfjs-core";

const e = env();
e.set("IS_TEST", true);

import "@tensorflow/tfjs-node";
