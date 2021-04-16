export interface Callbacks {
    onImageUpdated(): void;
}

type Props = {
    callbacks?: Callbacks,
};

export default function Capabilities(props: Props) {
    return (
        <p>TODO: Wire this up to buttons</p>
    );
}
