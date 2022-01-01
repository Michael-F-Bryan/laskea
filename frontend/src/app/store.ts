import {
    createSlice,
    configureStore,
    PayloadAction,
    createSelector,
} from "@reduxjs/toolkit";
import { Expression, Node, Value } from "./nodes";

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
            state.nodes[index].name = name;
        },
        setExpression: (
            state,
            action: PayloadAction<{ index: number; expr: Expression }>
        ) => {
            const { index, expr } = action.payload;
            state.nodes[index].expression = expr;
        },
        setResults: (state, action: PayloadAction<Value[]>) => {
            const results = action.payload;

            for (let i = 0; i < results.length; i++) {
                state.nodes[i].result = results[i];
            }
        },
    },
});

export const { addNode, removeNode, setName, setExpression, setResults } =
    nodesSlice.actions;

export const store = configureStore({ reducer: { nodes: nodesSlice.reducer } });

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;

export const selectExpressions = createSelector(
    (state: RootState) => state.nodes.nodes,
    (nodes: Node[]) => {
        const entries: Array<[string, Expression]> = nodes.map(n => [
            n.name,
            n.expression,
        ]);
        return Object.fromEntries(entries);
    }
);
