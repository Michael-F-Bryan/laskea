import { Button, TextField } from "@mui/material";
import { Expression } from "laskea-bindings";
import { useState } from "react";
import { useAppDispatch } from "../app/hooks";
import { setExpression } from "../app/store";

type Props = {
    index: number;
    expr: Extract<Expression, { type: "request" }>;
};

export default function RequestEditor({ index, expr }: Props) {
    const dispatch = useAppDispatch();
    const { url } = expr;
    const outdated = expr.response !== undefined && expr.error !== undefined;

    const setText = (url: string) => {
        dispatch(setExpression({ index, expr: { type: "request", url } }));
    };
    const onClick = async () => {
        const response = await sendRequest(url);
        dispatch(setExpression({ index, expr: response }));
    };

    return (
        <>
            <TextField
                value={url}
                placeholder="URL"
                onChange={e => setText(e.target.value)}
            />
            <Button onClick={onClick} color={outdated ? "error" : "info"}>
                Send
            </Button>
        </>
    );
}

async function sendRequest(url: string): Promise<Expression> {
    const options = {
        method: "get",
        headers: {
            "Content-Type": "application/json",
        },
    };

    try {
        const response = await fetch(url, options);
        const body = await response.json();

        return {
            type: "request",
            url,
            response: {
                status: response.status,
                status_text: response.statusText,
                url: response.url,
                body,
            },
        };
    } catch (e) {
        console.error(e);
        const error = e instanceof Error ? e.message : "Request failed";
        return { type: "request", url, error };
    }
}
