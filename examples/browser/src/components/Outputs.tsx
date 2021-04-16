import React, { Component } from "react";

type Props = {
    logs: string[],
};

export default function Outputs(props: Props) {
    const logMessages = props.logs.map(msg => (<p><code>{msg}</code></p>));

    return (
        <div className="outputs">
            <div className="logs">
                {logMessages}
            </div>
        </div>
    )
}
