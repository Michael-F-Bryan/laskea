import { TextField } from "@mui/material";
import { useAppDispatch } from "../app/hooks";
import { Request } from "../app/nodes";
import { setExpression } from "../app/store";

type Props = {
    index: number;
    expr: Request;
};

export default function RequestEditor({ index, expr }: Props) {
    const dispatch = useAppDispatch();
    const { url } = expr;
    const setText = (url: string) =>
        dispatch(setExpression({ index, expr: { ...expr, url } }));

    return (
        <>
            <TextField
                value={url}
                placeholder="url"
                onChange={e => setText(e.target.value)}
            />
        </>
    );
}
