import { RectangleSongBox, GeneralContextMenu } from "@components/index";
import { mouse_coOrds, contextMenuEnum, Song, contextMenuButtons } from "types";
import { useState, useRef, useEffect } from "react";
import "@styles/layouts/SearchSongs.scss";
import { ViewportList } from 'react-viewport-list';
import { local_songs_db } from "@database/database";
import { useSearchStore } from "store";

const SearchSongs = () => {
    const [selected, setSelectedSong] = useState<number>(0);
    const [co_ords, setCoords] = useState<mouse_coOrds>({xPos: 0, yPos: 0});
    const [songMenuToOpen, setSongMenuToOpen] = useState< Song | null>(null);
    const { query } = useSearchStore((state) => { return { query: state.query}; });
    const [SongList, setSongLists] = useState<Song[]>([]);

    const ref = useRef<HTMLDivElement | null>(null);

    function selectThisSong(index: number){ setSelectedSong(index); }

    function setMenuOpenData(key: number, n_co_ords: {xPos: number; yPos: number;}){
        setCoords(n_co_ords);
        const matching_song = SongList.find(song => { return song.id === key; })
        setSongMenuToOpen(matching_song ? matching_song : null);
    }

    function chooseOption(arg: contextMenuButtons){
    
    }

    useEffect(() => {
        const resetSongLists = () => {
            local_songs_db.songs.where("title").startsWithIgnoreCase(query).toArray().then((res) => { setSongLists(res);});
        }

        resetSongLists();
    }, [query])

    return (
        <div className="SearchSongs">
            <div className="SearchSongs-container" ref={ref}>
                <ViewportList viewportRef={ref} items={SongList}>
                    {(song, index) => (
                        <RectangleSongBox 
                        key={song.id}
                        keyV={song.id}
                        index={index + 1} 
                        cover={song.cover} 
                        songName={song.title} 
                        artist={song.artist}
                        length={song.duration} 
                        year={song.year}
                        selected={selected === index + 1 ? true : false}
                        selectThisSong={selectThisSong}
                        setMenuOpenData={setMenuOpenData}/>
                    )}
                </ViewportList>
                <div className="AllTracks_container_bottom_margin"/>
            </div>
            {
                songMenuToOpen && (
                    <div className="SearchSongs-ContextMenu-container" 
                    onClick={() => {
                        setSongMenuToOpen(null);
                        setCoords({xPos: 0, yPos: 0});
                    }} 
                    onContextMenu={(e) => {
                        e.preventDefault();
                        setSongMenuToOpen(null);
                        setCoords({xPos: 0, yPos: 0});
                    }}
                    >
                        <GeneralContextMenu 
                            xPos={co_ords.xPos} 
                            yPos={co_ords.yPos} 
                            title={songMenuToOpen.title}
                            CMtype={contextMenuEnum.SongCM}
                            chooseOption={chooseOption}/>
                    </div>
                )
            }
        </div>
    )
}

export default SearchSongs