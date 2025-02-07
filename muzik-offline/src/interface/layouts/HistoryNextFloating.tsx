import { motion } from "framer-motion";
import { FunctionComponent, useEffect, useReducer, useRef, useState } from "react";
import "@styles/layouts/HistoryNextFloating.scss";
import { AddSongToPlaylistModal, EditPropertiesModal, GeneralContextMenu, PropertiesModal, SongCardResizableDraggable } from "@components/index";
import { Song, contextMenuButtons, contextMenuEnum } from "@muziktypes/index";
import { local_albums_db, local_songs_db } from "@database/database";
import { useNavigate } from "react-router-dom";
import { useUpcomingSongs, useHistorySongs, useSavedObjectStore, reducerType } from "@store/index";
import { UpcomingHistoryState, upcomingHistoryReducer } from "@store/reducerStore";
import { closeContextMenu, closeEditPropertiesModal, closePlaylistModal, closePropertiesModal } from "@utils/reducerUtils";
import { addThisSongToPlayNext, addThisSongToPlayLater, playThisSongFromQueue } from "@utils/playerControl";
import { onDragEnd } from "@utils/index";

type HistoryNextFloatingProps = {
    FloatingHNState: boolean;
    toggleFloatingHNState: () => void;
}

const variants={
    open: {right: "16px"},
    closed: {right: "-300px"},
}

const HistoryNextFloating : FunctionComponent<HistoryNextFloatingProps> = (props: HistoryNextFloatingProps) => {
    const [state , dispatch] = useReducer(upcomingHistoryReducer, UpcomingHistoryState); 
    const {SongQueueKeys} = useUpcomingSongs((state) => { return { SongQueueKeys: state.queue}; });
    const {SongHistoryKeys} = useHistorySongs((state) => { return { SongHistoryKeys: state.queue}; });
    const {local_store} = useSavedObjectStore((state) => { return { local_store: state.local_store}; });
    const scrollRefUpcoming = useRef<HTMLDivElement | null>(null);
    const scrollRefHistory = useRef<HTMLDivElement | null>(null);
    const [upcomingPosition, setUpcomingPosition] = useState<"Top" | "Middle" | "Bottom">("Top");
    const [historyPosition, setHistoryPosition] = useState<"Top" | "Middle" | "Bottom">("Top");

    const navigate = useNavigate();

    function setMenuOpenData__SongQueue(key: number, n_co_ords: {xPos: number; yPos: number;}){
        dispatch({ type: reducerType.SET_COORDS, payload: n_co_ords });
        const matching_song = state.SongQueue.find((song, index) => { 
            if(song.id === key)dispatch({ type: reducerType.SET_KEY_INDEX_SONG_QUEUE, payload: {key,index,queueType: "SongQueue"} });
            return song.id === key; 
        });
        dispatch({ type: reducerType.SET_SONG_MENU, payload: matching_song ? matching_song : null });
    }

    function setMenuOpenData_SongHistory(key: number, n_co_ords: {xPos: number; yPos: number;}){
        dispatch({ type: reducerType.SET_COORDS, payload: n_co_ords });
        const matching_song = state.SongHistory.find((song, index) => { 
            if(song.id === key)dispatch({ type: reducerType.SET_KEY_INDEX_SONG_QUEUE, payload: {key,index,queueType: "SongHistory"} });
            return song.id === key; 
        });
        dispatch({ type: reducerType.SET_SONG_MENU, payload: matching_song ? matching_song : null });
    }

    function chooseOption(arg: contextMenuButtons){
        if(arg === contextMenuButtons.ShowInfo){ dispatch({ type: reducerType.SET_PROPERTIES_MODAL, payload: true}); }
        else if(arg === contextMenuButtons.AddToPlaylist){ dispatch({ type: reducerType.SET_PLAYLIST_MODAL, payload: true}); }
        else if(arg === contextMenuButtons.EditSong){ dispatch({ type: reducerType.SET_EDIT_SONG_MODAL, payload: true}); }
        else if(arg === contextMenuButtons.PlayNext && state.songMenuToOpen){ 
            addThisSongToPlayNext([state.songMenuToOpen.id]);
            closeContextMenu(dispatch); 
        }
        else if(arg === contextMenuButtons.PlayLater && state.songMenuToOpen){ 
            addThisSongToPlayLater([state.songMenuToOpen.id]);
            closeContextMenu(dispatch); 
        }
        else if(arg === contextMenuButtons.Play && state.songMenuToOpen){
            playThisSongFromQueue(state.kindex_sq.key, state.kindex_sq.index, state.kindex_sq.queueType);
            dispatch({ type: reducerType.SET_KEY_INDEX_SONG_QUEUE, payload: {key: -1, index: -1, queueType: "SongQueue"} });
            closeContextMenu(dispatch); 
        }
    }

    async function navigateTo(key: number, type: "artist" | "song", queueType: string){
        const relatedSong = queueType === "SongHistory" ? state.SongHistory.find((value) => value.id === key)
        : state.SongQueue.find((value) => value.id === key);
        if(!relatedSong)return;
        if(type === "song"){
            const albumres = await local_albums_db.albums.where("title").equals(relatedSong.album).first();
            if(albumres === undefined)return;
            navigate(`/AlbumDetails/${albumres.key}/undefined`);
        }
        else if(type === "artist"){
            navigate(`/ArtistCatalogue/${relatedSong.artist}`); 
        }
    }

    async function setLists(){
        const limit = Number.parseInt(local_store.UpcomingHistoryLimit);
        const sqkeys = SongQueueKeys.slice(0, limit);
        const hskeys = SongHistoryKeys.slice(SongHistoryKeys.length - limit, SongHistoryKeys.length);

        const USsongs = await local_songs_db.songs.where("id").anyOf(sqkeys).toArray();
        const HSsongs = await local_songs_db.songs.where("id").anyOf(hskeys).toArray();

        const USsongsOrdered = sqkeys.map(key => USsongs.find(item => item.id === key));
        const HSsongsOrdered = hskeys.map(key => HSsongs.find(item => item.id === key));

        dispatch({ type: reducerType.SET_SONG_QUEUE, payload: USsongsOrdered as Song[] });
        dispatch({ type: reducerType.SET_SONG_HISTORY, payload: HSsongsOrdered as Song[] });
    }

    const handleScrollUpcoming = () => {
        if (scrollRefUpcoming.current) {
        const { scrollTop, scrollHeight, clientHeight } = scrollRefUpcoming.current;

        if (scrollTop === 0) {setUpcomingPosition("Top");}
        else if (Math.ceil(scrollTop) + clientHeight >= scrollHeight) {setUpcomingPosition("Bottom");}
        else {setUpcomingPosition("Middle");}
        }
    };

    const handleScrollHistory = () => {
        if (scrollRefHistory.current) {
        const { scrollTop, scrollHeight, clientHeight } = scrollRefHistory.current;

        if (scrollTop === 0) {setHistoryPosition("Top");}
        else if (Math.ceil(scrollTop) + clientHeight >= scrollHeight) {setHistoryPosition("Bottom");}
        else {setHistoryPosition("Middle");}
        }
    };

    useEffect(() => {
        const currentUpcomingRef = scrollRefUpcoming.current;
        const currentHistoryRef = scrollRefHistory.current;
        if (currentUpcomingRef)currentUpcomingRef.addEventListener("scroll", handleScrollUpcoming);
        if (currentHistoryRef)currentHistoryRef.addEventListener("scroll", handleScrollHistory);

        setLists()
        return () => {
            if (currentUpcomingRef)currentUpcomingRef.removeEventListener("scroll", handleScrollUpcoming);
            if (currentHistoryRef)currentHistoryRef.removeEventListener("scroll", handleScrollHistory);
        };
    }, [SongQueueKeys, SongHistoryKeys])

    return (
        <>
            <motion.div className="HistoryNextFloating"
                animate={props.FloatingHNState ? "open" : "closed"}
                variants={variants}
                transition={!local_store.Animations ? {} : { type: "spring", stiffness: 100, damping: 15 }}
            >
                { props.FloatingHNState &&
                    <>
                        {
                            state.selectedView === "Upcoming_tab" ?
                            <div 
                                ref={scrollRefUpcoming}
                                onScroll={handleScrollUpcoming}
                                className={
                                    upcomingPosition === "Top" ? "top_view" : 
                                    upcomingPosition === "Bottom" ? "bottom_view" :
                                    "Upcoming_view"
                                }>
                                <SongCardResizableDraggable 
                                    SongQueue={state.SongQueue} 
                                    queueType={"SongQueue"} 
                                    onDragEnd={onDragEnd} 
                                    setMenuOpenData={setMenuOpenData__SongQueue} 
                                    playThisSong={playThisSongFromQueue} 
                                    navigateTo={navigateTo} />                        
                            </div>
                            :
                            <div 
                                ref={scrollRefHistory}
                                onScroll={handleScrollHistory}
                                className={
                                    historyPosition === "Top" ? "top_view" : 
                                    historyPosition === "Bottom" ? "bottom_view" :
                                    "History_view"
                                }>
                                <SongCardResizableDraggable 
                                    SongQueue={state.SongHistory} 
                                    queueType={"SongHistory"} 
                                    onDragEnd={onDragEnd} 
                                    setMenuOpenData={setMenuOpenData_SongHistory} 
                                    playThisSong={playThisSongFromQueue} 
                                    navigateTo={navigateTo} /> 
                            </div>
                        }
                        <div className="HistoryUpcoming_tabs">
                            <motion.div className="Upcoming_tab" whileTap={{scale: 0.98}}
                            onMouseUp={() => dispatch({ type: reducerType.SET_SELECTED_VIEW, payload: "Upcoming_tab" })}>
                                {state.selectedView === "Upcoming_tab" && <motion.div layoutId="active-pill" className="selected"/>}
                                <h3>Upcoming</h3>
                            </motion.div>
                            <motion.div className="History_tab" whileTap={{scale: 0.98}}
                            onMouseUp={() => dispatch({ type: reducerType.SET_SELECTED_VIEW, payload: "History_tab" })}>
                                {state.selectedView === "History_tab" && <motion.div layoutId="active-pill" className="selected"/>}
                                <h3>History</h3>
                            </motion.div>
                        </div>
                    </>
                }
            </motion.div>
            {
                state.songMenuToOpen && state.co_ords.xPos != 0 && state.co_ords.yPos != 0 && (
                    <div className="HistoryNextFloating-ContextMenu-container" 
                        onClick={(e) => closeContextMenu(dispatch, e)} onContextMenu={(e) => closeContextMenu(dispatch, e)}>
                        <GeneralContextMenu 
                            xPos={state.co_ords.xPos} 
                            yPos={state.co_ords.yPos} 
                            title={state.songMenuToOpen.name}
                            CMtype={contextMenuEnum.SongCM}
                            chooseOption={chooseOption}/>
                    </div>
                )
            }
            <PropertiesModal isOpen={state.isPropertiesModalOpen} song={state.songMenuToOpen!} closeModal={() => closePropertiesModal(dispatch)} />
            <EditPropertiesModal isOpen={state.isEditingSongModalOpen} songID={state.songMenuToOpen ? state.songMenuToOpen.id : -1} closeModal={() => closeEditPropertiesModal(dispatch)} />
            <AddSongToPlaylistModal isOpen={state.isPlaylistModalOpen} songPath={state.songMenuToOpen ? state.songMenuToOpen.path : ""} closeModal={() => closePlaylistModal(dispatch)} />
        </>
    )
}

export default HistoryNextFloating