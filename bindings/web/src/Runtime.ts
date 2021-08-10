import { KnownCapabilities, KnownOutputs, } from "./constants";

export default class Runtime {
    parameters;
    data;
    instance;
    models;

    constructor(instance, params, models, data) {
        this.instance = instance;
        this.parameters = params;
        this.data = data;
        this.models = models;
    }

    setInput(input) {

    }
    getCapabilities() {
        let cap = [];
        this.parameters.forEach((param) => {
            if (param.function == "request_capability") {
                cap.push(param);
            }
        });
        return cap;
    }

    getParameters() {
        let params = [];
        this.parameters.forEach((param) => {
            if (param.function == "request_capability_set_param") {
                params.push(param);
            }
        });
        return params;
    }

    getModels() {
        return this.models;
    }

    static async load(mod, imports, modelConstructor = loadTensorflowJSModel) {
        let parameters = [];
        let data = { "input": [], "output": [] };

        let memory;
        const [hostFunctions, finaliseModels, getParams, id] = importsToHostFunctions(imports, () => memory, () => parameters, () => data, modelConstructor);

        const instance = await WebAssembly.instantiate(mod, hostFunctions);
        if (!isRuneExports(instance.instance.exports)) {
            throw new Error("Invalid Rune exports");
        }

        memory = instance.exports.memory;

        console.log("manifest!!!");
        instance.exports._manifest();
        // now we've asked for all the models to be loaded, let's wait until
        // they are done before continuing
        await finaliseModels();
        return new Runtime(instance, parameters, id, data);
    }

    manifest() {
        //return 1;
        return this.exports._manifest();
    }



    call() {
        this.exports._call(0, 0, 0);
    }

    get parameters() {
        return this.params;
    }

    get exports() {
        // Note: checked inside Runtime.load() and exports will never change.
        return this.instance.exports;
    }
}
function constructFromNameTable(nextId, nameTable, constructors) {
    const instances = new Map();
    function create(type) {
        const name = nameTable[type];
        if (!name) {
            throw new Error(`type ${type} is unknown`);
        }
        const constructor = constructors[name];
        if (!constructor) {
            throw new Error(`No constructor for type ${type} called \"${name}\"`);
        }
        const instance = constructor();
        const id = nextId();
        instances.set(id, instance);
        return id;
    }
    return [instances, create];
}
function importsToHostFunctions(imports, getMemory, getParameters, getData, modelConstructor) {
    const memory = () => {
        const m = getMemory();
        if (!m)
            throw new Error("WebAssembly memory wasn't initialized");

        return new Uint8Array(m.buffer);
    };
    const parameters = () => {
        const p = getParameters();
        if (!p)
            throw new Error("Parameters wasn't initialized");

        return p;
    };
    const data = () => {
        const d = getData();
        if (!d)
            throw new Error("Data wasn't initialized");

        return d;
    }
    const ids = counter();
    const [outputs, createOutput] = constructFromNameTable(ids, KnownOutputs, imports.outputs);
    const [capabilities, createCapability] = constructFromNameTable(ids, KnownCapabilities, imports.capabilities);
    const pendingModels = [];
    const models = new Map();
    const utf8 = new TextDecoder();
    // Annoyingly, this needs to be an object literal instead of a class.
    const env = {
        _debug(msg, len) {
            console.log("_debug");
            const raw = memory().subarray(msg, msg + len);
            const message = utf8.decode(raw);
            console.log(message);
        },
        request_output(type) {
            console.log("request output");
            return createOutput(type);
        },
        consume_output(id, buffer, len) {
            console.log("consume_output");
            const output = outputs.get(id);
            if (output) {
                const data = memory().subarray(buffer, buffer + len);
                output.consume(data);
            }
            else {
                throw new Error("Invalid output");
            }
        },
        request_capability(type) {
            parameters().push({ "function": "request_capability", "id": type, "capability": KnownCapabilities[type] })
            return createCapability(type);
        },
        request_capability_set_param() {
            //let bytes = [];
            const id = arguments[0];
            const key = memory().subarray(arguments[1], arguments[1] + arguments[2]);
            const value = memory().subarray(arguments[3], arguments[3] + arguments[4]);
            const valueType = KnownOutputs[arguments[5]];
            parameters().push({ "function": "request_capability_set_param", "id": id, "key": String.fromCharCode.apply(String, key), "value": new Uint32Array(value)[0], "valueType": valueType })
        },
        request_provider_response(buffer, len, id) {
            console.log("request_provider_response");
            const cap = capabilities.get(id);
            if (!cap) {
                throw new Error("Invalid capabiltiy");
            }
            const dest = memory().subarray(buffer, buffer + len);
            cap.generate(dest);
        },
        async tfm_preload_model(data, len, _) {
            console.log("model:", data, len);
            const modelData = memory().subarray(data, data + len);

            const pending = modelConstructor(modelData, getParameters());
            const id = ids();
            pendingModels.push(pending.then(model => [id, model]));
            console.log(id, pendingModels);
            return id;
        },
        tfm_model_invoke(id, inputPtr, inputLen, outputPtr, outputLen) {

            console.log("model invoke ", id);
            const model = models.get(id + 1);
            if (!model) {
                throw new Error("Invalid model");
            }
            const input = memory().subarray(inputPtr, inputPtr + inputLen);
            const output = memory().subarray(outputPtr, outputPtr + outputLen);
            model.transform(input, output);
        },
    };
    async function synchroniseModelLoading() {
        console.log("synchroniseModelLoading");
        const loadedModels = await Promise.all(pendingModels);
        console.log("synchroniseModelLoading done", loadedModels);
        pendingModels.length = 0;
        loadedModels.forEach(([id, model]) => {
            console.log("Setting Model", id, model);
            models.set(id, model);
        });
    }
    return [{ env }, synchroniseModelLoading, parameters, models];
}

function counter() {
    let value = 0;
    return () => value++;
}
function isRuneExports(obj: any) {
    return (obj &&
        obj.memory instanceof WebAssembly.Memory &&
        obj._call instanceof Function &&
        obj._manifest instanceof Function);
}

async function loadTensorflowJSModel(bytes, parameters = []) {
    console.log("running with input:", bytes, " and parameters:", parameters);
    let utf16String = '';
    var len = bytes.byteLength;
    for (var i = 0; i < len; i++) {
        utf16String += String.fromCharCode(bytes[i]);
    }
    let decodedString = decodeURIComponent(escape(utf16String));

    await modelToIndexedDB(decodedString);
    const model_name = "imagenet_mobilenet_v2";
    const model = await tf.loadGraphModel('indexeddb://' + model_name);
    return {
        transform(input, output) {
            let capabilities = getCapabilities(parameters);
            let params = getParameters(parameters);

            // IMAGE
            if (capabilities.includes("image")) {
                var float32Input = Array.from(input).map(function (element) {
                    return element * 1.0 / 255;
                });
                console.log("Image float32Input:", float32Input);
                //pub enum PixelFormat
                if (params["pixel_format"] == 3) {
                    //GrayScale =  3
                    input = tf.tensor2d(float32Input, [params["width"], params["height"]]).expandDims(0);
                } else {
                    //RGB = 0,
                    //BGR = 1,
                    //YUV = 2,
                    input = tf.tensor3d(float32Input, [params["width"], params["height"], 3]).expandDims(0);
                }
            }

            const out = model.predict(input);
            const result = out.dataSync();
            output.set(result);
        }

    }
}

function getCapabilities(parameters) {
    let cap = [];
    parameters.forEach((param) => {
        if (param.function == "request_capability") {
            cap.push(param.capability);
        }
    });
    return cap;
}

function getParameters(parameters) {
    let params = {};
    parameters.forEach((param) => {
        if (param.function == "request_capability_set_param") {
            params[param.key] = param.value;
        }
    });
    return params;
}

async function modelToIndexedDB(model_bytes) {
    var data = JSON.parse(LZString.decompressFromUTF16(model_bytes));
    var DBOpenRequest = window.indexedDB.open("tensorflowjs", 1);
    let successes = 0;
    DBOpenRequest.onupgradeneeded = function (event) {
        const db = DBOpenRequest.result;
        var objectStore = db.createObjectStore("models_store", {
            "keyPath": "modelPath"
        });
        var objectInfoStore = db.createObjectStore("model_info_store", {
            "keyPath": "modelPath"
        });

    }
    DBOpenRequest.onsuccess = function (event) {
        const db = DBOpenRequest.result;
        data.models_store.modelArtifacts.weightData = new Uint32Array(data.weightData).buffer;
        var objectStore = db.transaction("models_store", "readwrite").objectStore("models_store");
        var objectStoreRequest = objectStore.put(data["models_store"]);
        console.log("models_store", data["models_store"]);
        objectStoreRequest.onsuccess = function (event) {
            successes++;
        }
        var objectInfoStore = db.transaction("model_info_store", "readwrite").objectStore("model_info_store");
        var objectInfoStoreRequest = objectInfoStore.put(data["model_info_store"]);
        console.log(data["model_info_store"]);
        objectInfoStoreRequest.onsuccess = function (event) {
            successes++;
        }
    }
    while (successes < 2) {
        await new Promise(r => setTimeout(r, 100));
    }
    console.log("Imported from json!", successes)
    return true;
}

function getInt32Bytes(x) {
    var bytes = [];
    var i = 4;
    do {
        bytes[--i] = x & (255);
        x = x >> 4;
    } while (i)
    return bytes;
}
function string2Bin(str) {
    var result = [];
    for (var i = 0; i < str.length; i++) {
        result.push(str.charCodeAt(i));
    }
    return result;
}
