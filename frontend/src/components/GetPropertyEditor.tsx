import { TextField } from "@mui/material";
import { Expression } from "laskea-bindings";
import { useAppDispatch } from "../app/hooks";
import { setExpression } from "../app/store";

type Props = {
    index: number;
    expr: Extract<Expression, { type: "get-property" }>;
};

export default function GetPropertyEditor({ index, expr }: Props) {
    const dispatch = useAppDispatch();
    const { target, field } = expr;

    const setInput = (input: string) =>
        dispatch(setExpression({ index, expr: { ...expr, target } }));
    const setValue = (field: string) =>
        dispatch(setExpression({ index, expr: { ...expr, field } }));

    return (
        <>
            <TextField
                value={target}
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
