import React, { ChangeEvent } from "react";
import Capabilities, { Callbacks } from './components/Capabilities';
import { Runtime, Imports, Output } from "rune";
import Outputs from "./components/Outputs";

type State = {
  runtime?: Runtime,
  imports: Imports,
  logs: string[],
};

type Props = {};

export default class App extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    const logger = new LoggingOutput();
    logger.on("log", msg => {
      const { logs } = this.state;
      this.setState({ logs: [msg, ...logs], })
    });

    this.state = {
      imports: {
        capabilities: {},
        outputs: {
          serial: () => logger,
        }
      },
      runtime: undefined,
      logs: [],
    };
  }

  onFileInputChanged(e: ChangeEvent<HTMLInputElement>, imports: Imports) {
    if (e.target.files && e.target.files.length >= 1) {
      e.target.files[0].arrayBuffer()
        .then(rune => Runtime.load(rune, this.state.imports))
        .then(runtime => this.setState({ runtime, ...this.state }));
    }
  }

  get callbacks(): Callbacks {
    return {
      onImageUpdated(_width: number, _height: number, _pixels: Uint32Array) {
        throw new Error("TODO: Update imports.capabilities to use the new image");
      }
    };
  }

  evaluateRune() {
    const { runtime } = this.state;

    if (runtime) {
      runtime.call();
    } else {
      throw new Error("The runtime hasn't been initialized");
    }
  }

  render() {
    const canRun = this.state.runtime;

    return (
      <div className="App">
        <label className="custom-file-upload">
          Upload Rune
          <input type="file" accept=".rune"
            onChange={e => this.onFileInputChanged(e, this.state.imports)} />
        </label>
        <Capabilities callbacks={this.callbacks} />
        <button disabled={!canRun} onClick={() => this.evaluateRune()}>Run</button>
        <Outputs logs={this.state.logs} />
      </div>
    );
  }
}

type EventHandler<T> = (arg: T) => void;

class LoggingOutput implements Output {
  private decoder = new TextDecoder();
  private callbacks: EventHandler<string>[];

  consume(data: Uint8Array): void {
    const message = this.decoder.decode(data);

    for (const callback of this.callbacks) {
      callback(message);
    }
  }

  public on(event: "log", handler: EventHandler<string>) {
    this.callbacks.push(handler);
  }
}

