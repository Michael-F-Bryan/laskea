import { useDispatch, useSelector } from "react-redux";
import type { AppDispatch, RootState } from "./store";

export function useAppDispatch(): AppDispatch {
    return useDispatch();
}

export function useAppSelector<T>(selector: (rootState: RootState) => T): T {
    return useSelector(selector);
}
