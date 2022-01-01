export type StringConstant = {
    type: "string-constant";
    value: string;
};

export type Request = {
    type: "request";
    url: string;
    response?: ResponseValue;
    error?: string;
};
export type ResponseValue = {
    status: number;
    statusText: string;
    url: string;
    body: any;
};

export type Equals = {
    type: "equals";
    input: string;
    value: string;
};

export type GetProperty = {
    type: "get-property";
    input: string;
    field: string;
};

export type Expression = StringConstant | Request | Equals | GetProperty;

export type Node = {
    name: string;
    expression: Expression;
    result: Value;
};

export type Value =
    | null
    | boolean
    | number
    | string
    | { type: "object"; value: any }
    | { type: "error"; error: string };
