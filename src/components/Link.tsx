import { FC, ReactNode } from "react";
import { useAppContext } from "../App";




export const Link: FC<{
    page: ReactNode,
    children?: ReactNode
}> = (props) => {

    const { setPage } = useAppContext();

    return (<>
        <span style={{
            appearance: 'none',
        }}
            onClick={() => setPage(props.page)}>
            {props.children}
        </span>
    </>)
}