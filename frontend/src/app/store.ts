import { createSlice, configureStore, PayloadAction } from "@reduxjs/toolkit";
import { Expression, Node } from "./nodes";

type NodesState = {
    nodes: Node[];
};

const initialState: NodesState = {
    nodes: [],
};

const nodesSlice = createSlice({
    name: "nodes",
    initialState,
    reducers: {
        addNode: state => {
            const emptyNode: Node = {
                name: "",
                expression: {
                    type: "string-constant",
                    value: "",
                },
                result: null,
                errors: {},
            };
            state.nodes.push(emptyNode);
        },
        removeNode: (state, action: PayloadAction<number>) => {
            state.nodes.splice(action.payload);
        },
        setName: (
            state,
            action: PayloadAction<{ index: number; name: string }>
        ) => {
            const { index, name } = action.payload;
            const node = state.nodes[index];
            node.name = name;
            const otherNames = state.nodes
                .filter((n, i) => i != index)
                .map(n => n.name);

            if (name === "") {
                node.errors.name = "Required";
            } else if (!name.match(/^[a-zA-Z_][\w\d_-]*$/)) {
                node.errors.name = "Invalid Name";
            } else if (otherNames.includes(name)) {
                node.errors.name = "Duplicate Name";
            } else {
                node.errors.name = undefined;
            }
        },
        setExpression: (
            state,
            action: PayloadAction<{ index: number; expr: Expression }>
        ) => {
            const { index, expr } = action.payload;
            state.nodes[index].expression = expr;
        },
    },
});

export const { addNode, removeNode, setName, setExpression } =
    nodesSlice.actions;

export const store = configureStore({ reducer: { nodes: nodesSlice.reducer } });

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;
