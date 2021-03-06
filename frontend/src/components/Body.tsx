import { Button, Container } from "@mui/material";
import AddCircle from "@mui/icons-material/AddCircle";
import { useAppDispatch, useAppSelector } from "../app/hooks";
import { addNode, setResults } from "../app/store";
import NodeEditor from "./NodeEditor";
import { useEffect, useState } from "react";
import { Laskea } from "laskea-bindings";

export default function Body() {
    const dispatch = useAppDispatch();
    const { nodes } = useAppSelector(s => s.nodes);
    const [laskea] = useState(() => new Laskea());

    useEffect(() => {
        const results = laskea.evaluate(nodes);
        dispatch(setResults(results));
    }, [nodes, dispatch]);

    const renderedNodes = nodes.map((n, i) => {
        return <NodeEditor key={i} index={i} node={n} />;
    });

    return (
        <Container>
            <Button onClick={() => dispatch(addNode())}>
                <AddCircle fontSize="large" />
            </Button>

            {renderedNodes}
        </Container>
    );
}
