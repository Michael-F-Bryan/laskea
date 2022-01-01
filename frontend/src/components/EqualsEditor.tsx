import { TextField } from "@mui/material";
import { useAppDispatch } from "../app/hooks";
import { Equals } from "../app/nodes";
import { setExpression } from "../app/store";

type Props = {
    index: number;
    expr: Equals;
};

export default function EqualsEditor({ index, expr }: Props) {
    const dispatch = useAppDispatch();
    const { value } = expr;

    const setInput = (input: string) =>
        dispatch(setExpression({ index, expr: { ...expr, input } }));
    const setValue = (value: string) =>
        dispatch(setExpression({ index, expr: { ...expr, value } }));

    return (
        <>
            <TextField
                value={value}
                placeholder="Target"
                onChange={e => setInput(e.target.value)}
            />
            <TextField
                value={value}
                placeholder="Value"
                onChange={e => setValue(e.target.value)}
            />
        </>
    );
}
