import { TextField } from "@mui/material";
import { Expression } from "laskea-bindings";
import { useAppDispatch } from "../app/hooks";
import { setExpression } from "../app/store";

type Props = {
    index: number;
    expr: Extract<Expression, { type: "string" }>;
};

export default function StringConstantEditor({ index, expr }: Props) {
    const dispatch = useAppDispatch();
    const { value } = expr;

    const setText = (text: string) => {
        dispatch(setExpression({ index, expr: { ...expr, value: text } }));
    };

    return (
        <TextField
            value={value}
            placeholder="constant"
            onChange={e => setText(e.target.value)}
        />
    );
}
