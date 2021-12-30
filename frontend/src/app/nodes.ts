export type StringConstant = {
    type: "string-constant";
    value: string;
};

export type Request = {
    type: "request";
    url: string;
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
    errors: Errors;
};

export type Errors = {
    name?: string;
};

export type Value = null | boolean | string | { [key: string]: Value };
