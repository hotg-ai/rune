import { InitialOptionsTsJest } from "ts-jest/dist/types";

const config: InitialOptionsTsJest = {
  preset: "ts-jest/presets/js-with-ts",
  testEnvironment: "node",
  roots: ["src"],
  setupFilesAfterEnv: ["<rootDir>/src/setupTests.ts"],
};

export default config;
