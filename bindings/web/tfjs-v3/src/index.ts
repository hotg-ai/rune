export { loadTensorFlowJS } from "./load";

import { loadTensorFlowJS } from "./load";
import { mimetypes } from "@hotg-ai/rune/builtin";

export default {
    mimetype: mimetypes.tfjs,
    load: loadTensorFlowJS,
} as const;
