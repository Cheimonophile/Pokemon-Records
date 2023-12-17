import { FC, Fragment } from "react"
import { PAGES, PageKey, useAppContext } from "../App"




export const Nav: FC<{}> = () => {

    const { currentPage, setCurrentPage } = useAppContext()


    return (
        <div className="flex-none flex flex-row px-2 py-1 gap-2">
            {Object.keys(PAGES).map((page) => {

                /**
                 * If the page in the list is selected
                 */
                const selected = page === currentPage

                return (
                    <Fragment key={page}>
                        <button
                            className={`${selected ? 'underline' : ''}`}
                            onClick={() => setCurrentPage(page as PageKey)}>
                            {page}
                        </button>
                    </Fragment>
                )
            })}
        </div>
    )
}