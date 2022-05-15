import { Node, Runtime as RuntimeInterface } from "./loader";

export class Runtime implements RuntimeInterface {
  constructor(private graph: Record<number, Node>) {}

  infer(): Promise<void> {
    throw new Error("Method not implemented.");
  }
}
