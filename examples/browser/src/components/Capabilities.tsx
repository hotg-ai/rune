import React from "react";

export interface Callbacks {
    onImageUpdated(width: number, height: number, pixels: Uint32Array): void;
}

type Props = {
    callbacks?: Callbacks,
};

/**
 * A widget the user can use to provide a Rune with capability data.
 */
export default function Capabilities(props: Props) {
    return (
        <p>TODO: Wire this up to buttons and stuff</p>
    );
}
