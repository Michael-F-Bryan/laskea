import { useState } from "react";
import { Button, Paper, TextField } from "@mui/material";
import RemoveCircle from "@mui/icons-material/RemoveCircle";
import { Node } from "laskea-bindings";
import { useAppDispatch } from "../app/hooks";
import { setName, removeNode } from "../app/store";
import ExpressionEditor from "./ExpressionEditor";

type Props = {
    index: number;
    node: Node;
};

export default function NodeEditor({ index, node }: Props) {
    const dispatch = useAppDispatch();
    const [hover, setHover] = useState(false);

    const { name, expression } = node;

    return (
        <Paper
            elevation={3}
            sx={{ my: "5px", padding: "1em", display: "flex" }}
            onMouseEnter={() => setHover(true)}
            onMouseLeave={() => setHover(false)}
        >
            <TextField
                placeholder="Name"
                type="text"
                value={name}
                required
                onChange={e =>
                    dispatch(setName({ index, name: e.target.value }))
                }
            />

            <ExpressionEditor index={index} expr={expression} />

            <Button
                sx={{ ml: "auto", visibility: hover ? "visible" : "hidden" }}
                onClick={() => dispatch(removeNode(index))}
            >
                <RemoveCircle />
            </Button>
        </Paper>
    );
}
