import { TextField } from "@mui/material";
import { Expression } from "laskea-bindings";
import { useAppDispatch } from "../app/hooks";
import { setExpression } from "../app/store";

type Props = {
    index: number;
    expr: Extract<Expression, { type: "equals" }>;
};

export default function EqualsEditor({ index, expr }: Props) {
    const dispatch = useAppDispatch();
    const { target, value } = expr;

    const setTarget = (target: string) =>
        dispatch(setExpression({ index, expr: { ...expr, target } }));
    const setValue = (value: string) =>
        dispatch(setExpression({ index, expr: { ...expr, value } }));

    return (
        <>
            <TextField
                value={target}
                placeholder="Target"
                onChange={e => setTarget(e.target.value)}
            />
            <TextField
                value={value}
                placeholder="Value"
                onChange={e => setValue(e.target.value)}
            />
        </>
    );
}
