import { FC } from "react";
import { Battles } from "../pages/Battles";


export type Route = {
    name: string;
    component: FC<{}>
}



export const ROUTES = Object.freeze({
    Battles: <Battles />,
})