import "./Pagination.scss";
import { FunctionalComponent } from "preact";
import { StateUpdater, useState } from "preact/hooks";

export interface PaginationProps {
    setPage: StateUpdater<number>;
    pages: number;
    page: number;
}

export interface PaginationButtonProps {
    setPage: StateUpdater<number>;
    page: number;
    active: boolean;
    next?: boolean;
    prev?: boolean;
}

export const PaginationButton: FunctionalComponent<PaginationButtonProps> = ({
    page,
    active,
    setPage,
    next,
    prev,
}) => {
    return (
        <div
            className={`pagination-button ${
                active && !next && !prev ? "active" : ""
            } ${next || prev ? "utility" : ""}`}
            onClick={() => setPage(page)}
            disabled={active}>
            {next ? (
                <i className="fa-solid fa-caret-right" />
            ) : prev ? (
                <i className="fa-solid fa-caret-left" />
            ) : (
                page
            )}
        </div>
    );
};

export const Pagination: FunctionalComponent<PaginationProps> = ({
    pages,
    page,
    setPage,
}) => {
    const [offset, setOffset] = useState(0);
    const offsetted = 8;

    const setCurrent = ((n: number) => {
        setPage(n);
        setOffset(Math.max(0, n - 3));
    }) as StateUpdater<number>;

    return (
        <div className="pagination">
            <PaginationButton
                page={Math.max(0, page - 1)}
                setPage={setCurrent}
                active={Math.max(0, page - 1) == page}
                prev
            />

            {[...Array(offsetted)].map((_, i) => (
                <PaginationButton
                    page={i + offset}
                    setPage={setCurrent}
                    active={i + offset == page}
                />
            ))}

            <PaginationButton
                page={Math.min(pages, page + 1)}
                setPage={setCurrent}
                active={Math.min(pages, page + 1) == page}
                next
            />
        </div>
    );
};
