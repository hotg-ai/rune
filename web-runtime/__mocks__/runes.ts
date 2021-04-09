import { promises } from "fs";
import { join } from "path";


export default {
    sine: () => {
        const path = join(__dirname, "sine.rune");
        return promises.readFile(path);
    },
};
