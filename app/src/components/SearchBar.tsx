import {FunctionalComponent} from "preact";
import "./SearchBar.scss";

interface SearchBarProps {
    // eslint-disable-next-line no-unused-vars
    onSearch: (query: string) => void;
}

export const SearchBar: FunctionalComponent<SearchBarProps> = ({
                                                                   onSearch,
                                                               }) => {
    const handleInput = (event: Event) => {
        const target = event.target as HTMLInputElement;
        onSearch(target.value);
    };

    return (
        <div className="searchbar">
            <input type="text" placeholder="Search" onInput={handleInput} />
        </div>
    );
};
