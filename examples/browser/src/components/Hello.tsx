import * as React from "react";

export interface IHelloProps
{
    name: string;
}

export class Hello extends React.Component<IHelloProps, {}>
{
    public render(): JSX.Element
    {
        return (
            <>
                <h3>Oh hey - {this.props.name}</h3>
            </>
        );
    }
}
