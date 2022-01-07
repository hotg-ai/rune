"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.tfliteModel = exports.graphModel = exports.layersModel = void 0;
var fs_1 = __importDefault(require("fs"));
var path_1 = __importDefault(require("path"));
var projectRoot = path_1.default.join(__dirname, "../../../../..");
function layersModel() {
    var filename = path_1.default.join(__dirname, "sine-wave.tfjs-layers.zip");
    return fs_1.default.readFileSync(filename);
}
exports.layersModel = layersModel;
function graphModel() {
    var filename = path_1.default.join(__dirname, "sine-wave.tfjs-graph.zip");
    return fs_1.default.readFileSync(filename);
}
exports.graphModel = graphModel;
function tfliteModel() {
    var filename = path_1.default.join(projectRoot, "examples", "sine", "sinemodel.tflite");
    return fs_1.default.readFileSync(filename);
}
exports.tfliteModel = tfliteModel;
