import { MenuItem, Select } from "@mui/material";
import { Expression } from "../app/nodes";
import { useAppDispatch } from "../app/hooks";
import { setExpression } from "../app/store";
import StringConstantEditor from "./StringConstantEditor";
import RequestEditor from "./RequestEditor";
import EqualsEditor from "./EqualsEditor";
import GetPropertyEditor from "./GetPropertyEditor";

type Props = {
    index: number;
    expr: Expression;
};

type NarrowedExpression<T extends Expression["type"]> = Extract<
    Expression,
    { type: T }
>;
type Option<T extends Expression["type"]> = {
    name: string;
    render: (props: {
        index: number;
        expr: NarrowedExpression<T>;
    }) => JSX.Element;
    defaultValue: () => NarrowedExpression<T>;
};

type Options = {
    [T in Expression["type"]]: Option<T>;
};

const options: Options = {
    "string-constant": {
        name: "String",
        render: StringConstantEditor,
        defaultValue: () => ({ type: "string-constant", value: "" }),
    },
    request: {
        name: "HTTP",
        render: RequestEditor,
        defaultValue: () => ({
            type: "request",
            url: "https://httpbin.org/ip",
        }),
    },
    equals: {
        name: "Equals",
        render: EqualsEditor,
        defaultValue: () => ({ type: "equals", input: "", value: "" }),
    },
    "get-property": {
        name: "Property",
        render: GetPropertyEditor,
        defaultValue: () => ({ type: "get-property", input: "", field: "" }),
    },
};

export default function ExpressionEditor({ index, expr }: Props) {
    const dispatch = useAppDispatch();

    const changeExpressionType = (type: string) => {
        if (type == expr.type) {
            return;
        }

        dispatch(setExpression({ index, expr: options[type].defaultValue() }));
    };

    const menuItems = Object.entries(options).map(([type, { name }]) => {
        return (
            <MenuItem key={type} value={type}>
                {name}
            </MenuItem>
        );
    });

    const Render: (props: { index: number; expr: any }) => JSX.Element =
        options[expr.type].render;

    return (
        <>
            <Select
                value={expr.type}
                onChange={e => changeExpressionType(e.target.value)}
            >
                {menuItems}
            </Select>

            <Render index={index} expr={expr} />
        </>
    );
}
