import * as React from "react";
import { App } from "./App";
import { Hello } from "./components/Hello";

export interface IMainProps {
    app: App;
}

export class Main extends React.Component<IMainProps, {}>
{
    constructor(props: IMainProps) {
        super(props);
    }

    public render(): JSX.Element {
        return (
            <>
            <Hello name= { this.props.app.appName } />
            </>
        );
    }
}
