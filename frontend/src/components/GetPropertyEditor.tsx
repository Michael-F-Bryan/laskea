import { TextField } from "@mui/material";
import { useAppDispatch } from "../app/hooks";
import { Equals, GetProperty } from "../app/nodes";
import { setExpression } from "../app/store";

type Props = {
    index: number;
    expr: GetProperty;
};

export default function GetPropertyEditor({ index, expr }: Props) {
    const dispatch = useAppDispatch();
    const { input, field } = expr;

    const setInput = (input: string) =>
        dispatch(setExpression({ index, expr: { ...expr, input } }));
    const setValue = (field: string) =>
        dispatch(setExpression({ index, expr: { ...expr, field } }));

    return (
        <>
            <TextField
                value={input}
                placeholder="Target"
                onChange={e => setInput(e.target.value)}
            />
            <TextField
                value={field}
                placeholder="Field"
                onChange={e => setValue(e.target.value)}
            />
        </>
    );
}
