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
    ellipsis?: boolean;
}

export const PaginationButton: FunctionalComponent<PaginationButtonProps> = ({
    page,
    active,
    setPage,
    next,
    prev,
    ellipsis,
}) => {
    return (
        <div
            className={`pagination-button ${
                active && !next && !prev ? "active" : ""
            } ${next || prev ? "utility" : ""} ${ellipsis ? "ellipsis" : ""}`}
            onClick={() => setPage(page)}
            disabled={active}>
            {next ? (
                <i className="fa-solid fa-caret-right" />
            ) : prev ? (
                <i className="fa-solid fa-caret-left" />
            ) : ellipsis ? (
                <i className="fa-solid fa-ellipsis" />
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
        setPage(Math.max(0, n));
        setOffset(
            Math.max(0, n - 3) + offsetted > pages
                ? pages - offsetted
                : Math.max(0, n - 3)
        );
    }) as StateUpdater<number>;

    return (
        <div className="pagination">
            <PaginationButton
                page={Math.max(0, page - 1)}
                setPage={setCurrent}
                active={Math.max(0, page - 1) == page}
                prev
            />

            {[...Array(offsetted + 1)].map((_, i) =>
                i != 0 && (
                    <PaginationButton
                        page={i + offset}
                        setPage={setCurrent}
                        active={i + offset == page}
                        key={i + offset}
                    />
                )
            )}

            {offset + offsetted < pages ? (
                <>
                    <PaginationButton
                        page={-1}
                        setPage={(v) => void v}
                        active={false}
                        ellipsis
                    />

                    <PaginationButton
                        page={pages}
                        setPage={setCurrent}
                        active={pages == page}
                    />
                </>
            ) : (
                <></>
            )}

            <PaginationButton
                page={Math.min(pages, page + 1)}
                setPage={setCurrent}
                active={Math.min(pages, page + 1) == page}
                next
            />
        </div>
    );
};
