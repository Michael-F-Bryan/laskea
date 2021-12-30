import { render } from "react-dom";
import { Provider } from "react-redux";
import App from "./components/App";
import { store } from "./app/store";

render(
    <Provider store={store}>
        <App />
    </Provider>,
    document.querySelector("#root")
);
