import React, { ChangeEvent } from 'react';
import Capabilities, { Callbacks } from './Capabilities';
import { Runtime, Imports } from "rune";

type State = {
  runtime?: Runtime,
  imports: Imports,
};

export default class App extends React.Component<{}, State> {
  constructor(props: {}) {
    super(props);

    this.state = {
      imports: { capabilities: {}, outputs: {} },
      runtime: undefined,
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
      onImageUpdated() {
        throw new Error("TODO: Update imports.capabilities to use the new image");
      }
    };
  }

  render() {
    return (
      <div className="App">
        <label className="custom-file-upload">
          Upload Rune
          <input type="file" accept=".rune"
            onChange={e => this.onFileInputChanged(e, this.state.imports)} />
        </label>
        <Capabilities callbacks={this.callbacks} />
      </div>
    );
  }
}

